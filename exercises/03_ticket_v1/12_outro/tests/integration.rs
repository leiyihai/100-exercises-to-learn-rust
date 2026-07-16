use outro_02::Order;

// `tests` 目录中的文件只有在运行测试时才会被编译。
// 因此，我们不再需要 `#[cfg(test)]` 属性来进行条件编译——因为
// implied.

#[test]
fn test_order() {
    let mut order = Order::new("Rusty Book".to_string(), 3, 2999);

    assert_eq!(order.product_name(), "Rusty Book");
    assert_eq!(order.quantity(), &3);
    assert_eq!(order.unit_price(), &2999);
    assert_eq!(order.total(), 8997);

    order.set_product_name("Rust Book".to_string());
    order.set_quantity(2);
    order.set_unit_price(3999);

    assert_eq!(order.product_name(), "Rust Book");
    assert_eq!(order.quantity(), &2);
    assert_eq!(order.unit_price(), &3999);
    assert_eq!(order.total(), 7998);
}

// 验证测试
#[test]
#[should_panic]
fn test_empty_product_name() {
    Order::new("".to_string(), 3, 2999);
}

#[test]
#[should_panic]
fn test_long_product_name() {
    Order::new("a".repeat(301), 3, 2999);
}

#[test]
#[should_panic]
fn test_zero_quantity() {
    Order::new("Rust Book".to_string(), 0, 2999);
}

#[test]
#[should_panic]
fn test_zero_unit_price() {
    Order::new("Rust Book".to_string(), 3, 0);
}
