#[test]
fn grid_justify_content_end_with_padding_border() {
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node6 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node7 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node8 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                justify_content: Some(taffy::style::JustifyContent::End),
                grid_template_rows: vec![points(40f32), points(40f32), points(40f32)],
                grid_template_columns: vec![points(40f32), points(40f32), points(40f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(200f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(40f32),
                    right: taffy::style::LengthPercentage::Points(20f32),
                    top: taffy::style::LengthPercentage::Points(10f32),
                    bottom: taffy::style::LengthPercentage::Points(30f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(8f32),
                    right: taffy::style::LengthPercentage::Points(4f32),
                    top: taffy::style::LengthPercentage::Points(2f32),
                    bottom: taffy::style::LengthPercentage::Points(6f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5, node6, node7, node8],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 56f32, "x of node {:?}. Expected {}. Actual {}", node0, 56f32, location.x);
    assert_eq!(location.y, 12f32, "y of node {:?}. Expected {}. Actual {}", node0, 12f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1, 40f32, size.height);
    assert_eq!(location.x, 96f32, "x of node {:?}. Expected {}. Actual {}", node1, 96f32, location.x);
    assert_eq!(location.y, 12f32, "y of node {:?}. Expected {}. Actual {}", node1, 12f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2, 40f32, size.height);
    assert_eq!(location.x, 136f32, "x of node {:?}. Expected {}. Actual {}", node2, 136f32, location.x);
    assert_eq!(location.y, 12f32, "y of node {:?}. Expected {}. Actual {}", node2, 12f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 56f32, "x of node {:?}. Expected {}. Actual {}", node3, 56f32, location.x);
    assert_eq!(location.y, 52f32, "y of node {:?}. Expected {}. Actual {}", node3, 52f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node4, 40f32, size.height);
    assert_eq!(location.x, 96f32, "x of node {:?}. Expected {}. Actual {}", node4, 96f32, location.x);
    assert_eq!(location.y, 52f32, "y of node {:?}. Expected {}. Actual {}", node4, 52f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node5, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node5, 40f32, size.height);
    assert_eq!(location.x, 136f32, "x of node {:?}. Expected {}. Actual {}", node5, 136f32, location.x);
    assert_eq!(location.y, 52f32, "y of node {:?}. Expected {}. Actual {}", node5, 52f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node6).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node6, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node6, 40f32, size.height);
    assert_eq!(location.x, 56f32, "x of node {:?}. Expected {}. Actual {}", node6, 56f32, location.x);
    assert_eq!(location.y, 92f32, "y of node {:?}. Expected {}. Actual {}", node6, 92f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node7).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node7, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node7, 40f32, size.height);
    assert_eq!(location.x, 96f32, "x of node {:?}. Expected {}. Actual {}", node7, 96f32, location.x);
    assert_eq!(location.y, 92f32, "y of node {:?}. Expected {}. Actual {}", node7, 92f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node8).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node8, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node8, 40f32, size.height);
    assert_eq!(location.x, 136f32, "x of node {:?}. Expected {}. Actual {}", node8, 136f32, location.x);
    assert_eq!(location.y, 92f32, "y of node {:?}. Expected {}. Actual {}", node8, 92f32, location.y);
}
