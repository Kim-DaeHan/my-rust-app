#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    // :? 명시자를 집어넣는 것은 println! 에게 Debug라 불리우는 출력 포맷을 사용하고 싶다고 말해줌(구조체 정의 부분 바로 전에 #[derive(Debug)] 넣어줘야함.)
    println!("rect1 is {:?}", rect1);
    // 좀 더 이쁘게 출력 {:#?}
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
