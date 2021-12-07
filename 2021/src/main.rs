use futures::executor::block_on;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    day1::run();
    day2::run();
    day3::run();
    block_on(day4::run());
}