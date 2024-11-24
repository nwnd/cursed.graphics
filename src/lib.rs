use worker::*;

#[event(fetch)]
async fn fetch(request: Request, env: Env, context: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    // Split the path into segments
    let url = request.url().unwrap();
    let path_segments = url.path_segments();

    // Handle the sub-paths
    match path_segments {
        Some(path_segments) => match path_segments.collect::<Vec<&str>>().as_slice() {
            // Index
            [""] => {
                // Redirect the user to a random page from 1 to `__max__`
                let maximum_id = env
                    .kv("NAME_MAP")?
                    .get("__max__")
                    .text()
                    .await?
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                let random_id = rand::random::<u32>() % maximum_id + 1;

                Response::builder()
                    .with_status(302)
                    .with_header("Location", &format!("/{}", random_id))?
                    .ok("")
            }

            // Image subpage (addressed by numeric ID)
            [id] => {
                // Look up the corresponding image path
                let image_path = env.kv("NAME_MAP")?.get(id).text().await?;

                // If no result is found, this image doesn't exist
                if image_path.is_none() {
                    return Response::error("Not Found", 404);
                }

                // Serve the image
                Response::from_html(format!(
                    include_str!("../templates/image.html"),
                    image_id = id,
                    image_filename = image_path.unwrap()
                ))
            }

            _ => Response::error("Not Found", 404),
        },
        _ => Response::error("Not Found", 404),
    }
}
