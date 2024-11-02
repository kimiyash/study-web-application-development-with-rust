#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    #[fixture]
    fn 任意のフィクスチャ名() -> i32 {
        24
    }

    #[rstest]
    fn 任意の関数名(任意のフィクスチャ名: i32) {
        // 任意のフィクスチャ名には fn 任意のフィクスチャ名() の実行結果 24 が入る
        assert_eq!(任意のフィクスチャ名 * 2, 48);
    }
}
