use std::path::PathBuf;

use mac_address::{get_mac_address, MacAddress};

use crate::{file::{File, FileAttr}, op::OpType};

pub struct Machine {
    rendered: Option<String>,
    mac: MacAddress,
    db: PathBuf,
    logs: Vec<OpType>,
}

impl Machine {
    /// Creates a new [`Machine`].
    pub fn new(mac: MacAddress, db_path: PathBuf) -> Self {
        Self {
            rendered: None,
            mac,
            db: db_path,
            logs: vec![]
        }
    }
}

enum EnvErr {
    SyncExistConflict(String),
}

/// Keep track of the env, hence we can generate
/// human readily reports.
enum EnvStatus {
    Ready,
    Syncing,
    RunWithErr(Vec<EnvErr>),
}

/// Offliner env contains a local machine and several remote machines
/// All operation logs are managed by local_machine through this env.
struct Env {
    local_machine: Machine,
    sync_machines: Vec<Machine>,
    files: Vec<File>,
    root: PathBuf
}

impl Env {
    pub fn new(root: PathBuf) -> Self { // TODO: Repleace with Result
        let all_machines = Self::discover_all_machines(&root);

        // We should emit eliminate these unwraps by proper env errors.
        let mac = get_mac_address().unwrap().unwrap();

        let (mut local_machine, all_machines) = Self::find_local_machine(mac, all_machines);
        // If the roor doesn't have a db for local machine, we create one.
        if local_machine.is_none() {
            local_machine = Some(Self::create_local_machine(mac));
        }
        
        Self {
            local_machine: local_machine.unwrap(),
            sync_machines: all_machines,
            files: Self::scan_files(&root),
            root
        }
    }

    pub fn do_sync(&mut self) {
        unimplemented!()
    }

    fn scan_files(root: &PathBuf) -> Vec<File> { // TODO: Find target file recursively
        root.read_dir().unwrap()
            .filter_map(|x| x.ok())
            .map(|x| {
                let path = x.path();
                let attr = match x.file_name().into_string() {
                    Ok(name) => {
                        if name.ends_with(".htm") || name.ends_with(".html") {
                            FileAttr::new_html()
                        } else if name.ends_with(".pdf") {
                            FileAttr::new_pdf()
                        } else {
                            FileAttr::new_html() // TODO: Ignore these
                        }
                    }
                    Err(_) => todo!(),
                };

                File::new(attr, path)
            })
            .collect()
    }

    /// Filter the local machine by [MacAddress]
    fn find_local_machine(local_mac: MacAddress, machines: Vec<Machine>) -> (Option<Machine>, Vec<Machine>) {
        let local_mac = get_mac_address().unwrap().unwrap();
        let mut local_machine = None;
        let mut returned_machines = vec![];

        for machine in machines {
            if machine.mac == local_mac {
                local_machine = Some(machine);
            } else {
                returned_machines.push(machine);
            }
        }

        (local_machine, returned_machines)
    }

    fn create_local_machine(local_mac: MacAddress) -> Machine {
        // TODO: Fix empty db path
        Machine::new(local_mac, PathBuf::new())
    }

    /// Discover all machines from the `.machines` sub dir of  the root.
    /// Here we rely on [MacAddress] to keep uniqueness.
    fn discover_all_machines(root: &PathBuf) -> Vec<Machine> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Env;

    #[test]
    fn test_scan_files() {
        let files = Env::scan_files(&PathBuf::from("./test_dir"));
        // Enable with --nocapture
        println!("{:?}", files);
    }
}