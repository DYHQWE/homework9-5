struct Buffer<T> {
    member: Vec<T>,
}

impl<T> Buffer<T>
where
    T: std::ops::Add<Output = T> + Default + Clone,
{
    pub fn new() -> Self {
        Buffer { member: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.member.push(item);
    }

    pub fn sum(&self) -> T {
        self.member.iter().cloned().fold(Default::default(), |acc, x| acc + x)
    }
}

fn main() {
    //work1
    let mut buffer = Buffer::new();
    buffer.push(5);
    buffer.push(1);
    buffer.push(3);

    let result = buffer.sum();
    println!("Sum: {:?}", result);
    
    //work2
    let x = "string";
    let y = "stringa";
    let z = "str";
    let result = compare_string(x, y);

    if result{
        println!("'{}' > '{}'", x, y);
    } else {
        println!("'{}' <= '{}'", x, y);
    }

    let result = compare_string(y, z);

    if result{
        println!("'{}' > '{}'", y, z);
    } else{
        println!("'{}' <= '{}'", y, z);
    }

    //work3
    let input: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    
    let output: Vec<char> = input
        .iter()
        .skip(1) // 跳过'a'
        .map(|&x| x)
        .chain(std::iter::once('f')) // 添加新元素 'f'
        .collect();
    
    println!("{:?}", output);
}

fn compare_string(x: &str, y: &str) -> bool{
    let mut x_chars = x.chars();
    let mut y_chars = y.chars();

    loop{
        match (x_chars.next(), y_chars.next()){
            (Some(x_char), Some(y_char)) => { 
                if x_char > y_char {
                    return true;
                } else if x_char < y_char {
                    return false;
                }
                // 如果当前字符相等，继续比较下一个字符
            }
            (Some(_), None) => {
                // y 较短，但 x 还有字符，所以 x 较大
                return true;
            }
            (None, Some(_)) => {
                // x 较短，但 y 还有字符，所以 y 较大
                return false;
            }
            (None, None) => {
                // 达到字符串末尾，两个字符串相等
                return false;
            }
        }
    }
}