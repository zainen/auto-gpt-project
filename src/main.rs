mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;

fn main() {
    let user_req: String = get_user_response("what webserver are we building today?");

    dbg!(user_req);
}
