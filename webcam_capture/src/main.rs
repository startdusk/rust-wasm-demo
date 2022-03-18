use serde_json::json;
use sycamore::{futures::ScopeSpawnLocal, prelude::*};
use webcam_capture::VideoStream;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(|ctx| {
        view! {
            ctx,
            div(class="container p-2") {
                Video()
            }
        }
    })
}

#[component]
fn Video<G: Html>(ctx: ScopeRef) -> View<G> {
    let video_ref = ctx.create_node_ref();
    ctx.spawn_local(async move {
        let el = video_ref.get::<DomNode>().unchecked_into();
        let video_stream = VideoStream::new(el);
        video_stream
            .set_video_src(&json!({
                "audio": false,
                "video": {
                    "facingMode": "environment",
                    "width": 640,
                    "height": 480,
                }
            }))
            .await;
    });
    view! {ctx,
        div {
            video(
                ref=video_ref,
                class="border border-gray-400 rounded-lg",
                autoplay=true,
                width=640,
                height=480,
            )
        }
    }

    // 视频源 https://klxxcdn.oss-cn-hangzhou.aliyuncs.com/histudy/hrm/media/bg3.mp4
}
