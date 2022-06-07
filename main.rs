#[async_std::main]
async fn main() {
  async_add_logger(2, 8).await;
}

/** 非同期足し算 */
async fn async_add(left: i32, right: i32) -> i32 {
    return left + right;
}

/** 非同期足し算 ログを出力 */
async fn async_add_logger(left: i32, right: i32) {
    let result = async_add(left, right).await;
    println!("{} + {} = {}", left, right, result);
}
