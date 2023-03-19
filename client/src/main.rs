#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut display = x11::display::Display::open()?;

    let root = display.root().unwrap();

    // display
    //     .change_window_attributes(root)
    //     .background_pixmap(BackgroundPixmap::ParentRelative)
    //     .event_mask(
    //         EventMask::SubstructureNotify,
    //         EventMask::SubstructureRedirect,
    //     )
    //     .await?;

    // x11::request::ChangeWindowAttributes::new(root)
    //     .background_pixmap(BackgroundPixmap::ParentRelative)
    //     .event_mask(
    //         EventMask::SubstructureNotify,
    //         EventMask::SubstructureRedirect,
    //     )
    //     .send(display)
    //     .await?;

    let children = display.query_tree(root).await?;
    dbg!(children);

    x11::request::QueryTree::new(root).write(&mut display)?;

    let window = display.setup.resource_id_base + 1;
    x11::request::CreateWindow::new(
        0,
        window,
        root,
        100,
        50,
        500,
        250,
        5,
        x11::proto::Class::CopyFromParent,
        x11::proto::Visual::CopyFromParent,
    )
    .write(&mut display)?;

    x11::request::MapWindow::new(window).write(&mut display)?;

    loop {
        let message = display.next_message()?;
        println!("Message({:#?})", message);
    }
}
