use match_expression::{check_status, req_status};

fn main() {
    for status in req_status() {
        check_status(status);
    }
}
