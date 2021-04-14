use std::f64::consts::PI;

fn main() {
    // ----------------- 作业1 --------------------
    let red_light = TrafficLight::Red;
    let green_light = TrafficLight::Green;
    let yellow_light = TrafficLight::Yellow;
    println!("light are : red {} greed {} yellow {}", 
        red_light.time(), 
        green_light.time(),
        yellow_light.time()
    );
    // ----------------- 作业2 --------------------
    let list = &[32, 10, 21, 111];
    sum_u32_list(list);

    // ----------------- 作业3 --------------------
    // 见下方代码

}

// ----------------- 作业1 --------------------
// 问题：为枚举交通信号灯实现一个trait，trait里包含一个返回时间的方法，不同灯持续时间不同
// 定义枚举
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

// 实现枚举方法
impl TrafficLight {
    fn time(&self) -> u8 {
        match &self {
            TrafficLight::Red => 30,
            TrafficLight::Green => 60,
            TrafficLight::Yellow => 5
        }
        
    }
}

// ----------------- 作业2 --------------------
// 问题：实现一个函数，为u32类型的整数集合求和，参数类型为&[u32]，返回类型为Option<u32>，溢出时返回None
fn sum_u32_list(x: &[u32]) -> Option<u32> {
    let mut result: u32 = 0 ;
    for i in x.iter() {
        result += i;
    }
    if result > (2^31-1) {
        Some(result)
    } else {
        None
    }
}

// ----------------- 作业3 --------------------
// 问题：实现一个打印图形面积的函数，接收一个可以计算面积的类型作为参数，需要用到泛型和泛型约束
pub enum Shape {
    Rectangle{ width: f64, height: f64},
    Circle{ r: f64 },
    Triangle{ a: f64, b: f64, c:f64 },
}

pub trait Area {
    fn area (&self) -> f64;
}

impl Area for Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Circle{r} => r * r * PI,
            Shape::Triangle { a, b, c } => {
                let p = (a + b + c) / 2.0;
                (p * (p - a) * (p - b) * (p - c)).sqrt()
            }
        }
    }
}