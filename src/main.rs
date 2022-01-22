use minparse::minparse;

fn main(){
    println!("\t      procname: {}
              switches: {:?}
              subcommands: {:?}
              fields: {:?}",
            minparse::process_name(),
            minparse::switches(),
            minparse::subcommands(),
            minparse::fields()
        )

}