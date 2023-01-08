use crate::models::User;

use diesel::prelude::*;
use crate::establish_connection;

async fn get_user(external_id: String) -> Option<User> {
    let connection = &mut establish_connection();
    User.get(external_id.eq(external_id))
    
}