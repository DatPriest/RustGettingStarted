use tide::Endpoint;

async fn handle_get_users(cx: Context<Database>) -> EndpointResult {
    Ok(response::json(cx.app_data().get_all()))
}

async fn handle_get_user(cx: Context<Database>) -> EndpointResult {
    let id = cx.param("id").client_err()?;
    if let Some(user) = cx.app_data().get(id) {
        Ok(response::json(user))
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}

async fn handle_update_user(mut cx: Context<Database>) -> EndpointResult<()> {
    let user = (cx.body_json()).await?.client_err()?;
    let id = cx.param("id").client_err()?;

    if cx.app_data().set(id, user) {
        Ok(())
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}

async fn handle_create_user(mut cx: Context<Database>) -> EndpointResult<String> {
    let user = (cx.body_json()).await?.client_err()?;
    Ok(cx.app_data().insert(user).to_string())
}

async fn handle_delete_user(cx: Context<Database>) -> EndpointResult<String> {
    let id = cx.param("id").client_err()?;
    Ok(cx.app_data().delete(id).to_string())
}

fn main() {
    // We create a new application with a basic, local database
    // You can use your own implementation, or none: App::new(())
    let mut app = App::new(Database::default());
    app.at("/users")
        .post(handle_create_user)
        .get(handle_get_users);
    app.at("/users/:id")
        .get(handle_get_user)
        .patch(handle_update_user)
        .delete(handle_delete_user);

    app.serve("127.0.0.1:8000").unwrap();
}