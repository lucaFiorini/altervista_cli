mod schema;
mod front;
mod back;
mod model;

use diesel::Insertable;
use back::conn;


fn main(){
    conn::establish_connection();
    
}