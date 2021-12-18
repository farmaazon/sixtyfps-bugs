
sixtyfps::sixtyfps! {
    import { Button } from "./sixtyfps_widgets.60";

    Background := Rectangle {
        property<bool> is-horizontal;
        property<int> index;
        color: red;

        states [
            horizontal when is-horizontal: {
                width: 100px;
                height: 20px;
            }
            vertical when !is-horizontal: {
                height: 100px;
                width: 20px;
            }
        ]
    }

    WithLabel := Background {
        label := Text {}

        states [
            horizontal when is-horizontal: {
                label.text: "horizontal";
            }
            vertical when !is-horizontal: {
                label.text: "vertical";
            }
        ]
    }

    Main := Window {
        width: 200px;
        height: 200px;

        WithLabel {
            is-horizontal: false;
        }

        Button {
            x: 50px;
            y: 100px;
            width: 100px;
            height: 30px;
            text: "Click me!";
        }

        // // Uncomment to have a rust compilation error.
        // for i in 2: Rectangle {
        //     property<int> index: i;
        //     x: 20px * index;
        //     y: 150px;
        //     color: blue;
        //     width: 10px;
        //     height: 10px;
        // }
    }
}

fn main() {
    Main::new().run();
}
