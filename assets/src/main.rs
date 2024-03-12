#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use manganis::*;

//여기서는 에러가 나지만, 실제 실행해보면 잘 작동한다.
const ASSET: manganis::ImageAsset = manganis::mg!(image("./public/static/2.png")
    .size(52, 52)
    .format(ImageType::Avif)
    .preload());

// 이렇게 하면 사진의 크기와 형식을 커스텀 할 수 있어서, 웹에 맞춰서 용량을 줄여 성능을 높일 수 있다.

// 또한 image(경로)뿐 아니라, file(경로)를 이용해서 기타 파일도 에셋으로 받아올 수 있다. (css등등)

fn main() {
    launch(app)
}

fn app() -> Element {
    rsx! {
        img{src:"{ASSET}"}
    }
}
