use gotham_derive::StateData;

use gotham::{
    helpers::http::response::{create_response, create_empty_response},
    middleware::state::StateMiddleware,
    hyper::{Body, Response, StatusCode},
    pipeline::{single_pipeline, single_middleware},
    router::{builder::*, Router},
    state::{FromState, State}
};

use wakey::WolPacket;

use std::sync::{Arc, Mutex};
use log::{info, warn};

use crate::structure::{Config, RequestBody};

#[derive(Clone, StateData)]
struct WebState {
    state: Arc<Mutex<Config>>
}

fn wake(state: State) -> (State, Response<Body>) {
    let request_body = RequestBody::borrow_from(&state);

    let response = match WolPacket::from_string(&request_body.address, ':') {
        Ok(wol) => {
            match wol.send_magic() {
                Ok(_) => {
                    create_empty_response(&state, StatusCode::OK)
                },
                Err(_) => {
                    create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR)
                }
            }           
        },
        Err(_msg) => {
            create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    (state, response)
}

fn get_data(state: State) -> (State, Response<Body>) {

    let binding = WebState::borrow_from(&state).state.clone();
    let data = binding.lock().unwrap();

    let json_string = serde_json::to_string(&*data);
    let response = match json_string {
        Ok(value) => {
            create_response(&state, StatusCode::OK, "application/json".parse().unwrap(), value)
        },
        Err(_msg) => {
            create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    (state, response)
}

fn router(frontend_path: &'static str, state: Arc<Mutex<Config>>) -> Router {

    let web_state = WebState { state: state };
    let middleware = StateMiddleware::new(web_state);
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route.get("/").to_file(frontend_path.clone().to_owned() + "/index.html");
        route.get("/*").to_dir(frontend_path);
        route.get("/api/data").to(get_data);
        route.put("/api/wake/:address").with_path_extractor::<RequestBody>().to(wake);
    })
}

pub fn run(address: &'static str, frontend_path: &'static str, state: Arc<Mutex<Config>>) {
    let result = gotham::start(address, router(frontend_path, state));

    match result {
        Ok(_msg) => info!("Webserver started successfully"),
        Err(_msg) => warn!("Error starting web server")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use gotham::hyper::StatusCode;

    fn generate_test_server() -> TestServer {
        let frontend_path = "../frontend/build";
        let state: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
        let test_server = TestServer::new(router(frontend_path, state)).unwrap();

        return test_server;
    }

    #[test]
    fn index_get() {
        let test_server = generate_test_server();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn static_get() {
        let test_server = generate_test_server();
        let response = test_server
            .client()
            .get("http://localhost/static/css/index.css")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn static_test_get() {
        let test_server = generate_test_server();
        let response = test_server
            .client()
            .get("http://localhost/static/test.png")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}