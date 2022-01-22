use minparse::minparse;

fn main(){
    println!("\tprocname: {}
              switches: {:?}
              subcommands: {:?}
              fields: {:?}",
            minparse::process_name(),
            minparse::switches(),
            minparse::subcommands(),
            minparse::fields()
        )

}