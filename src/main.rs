use minparse::minparse;

fn main(){
    println!("\t\tprocname: {}
              switches: {:?}
              subcommands: {:?}", minparse::process_name(), minparse::switches(),  minparse::subcommands())

}