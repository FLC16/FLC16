use rand::prelude::*;
use std::sync::mpsc::Sender;
use bresenham::Bresenham;
use crate::text;

fn pop(s: &mut Vec<u8>) -> u8 {
    return s.pop().expect("Attempted to pop from the stack when no values where present");
}

fn push(s: &mut Vec<u8>, v: u8) {
    if s.len() == 256 {
        // troll the haters
        panic!("Virtual stack overflow");
    }
    return s.push(v);
}

pub fn bcode(
    bytes: &[u8],
    pix_buffer: &mut [[u8; 256]; 144],
    spawned: bool,
    spawn_index: usize,
    mut heap: Vec<u8>,
    mut stack: Vec<u8>,
    mut routines: Vec<u32>,
    tx: &Sender<String>,
    mut palette: &mut [[u8; 4]; 16]
) -> (Vec<u32>, Vec<u8>, Vec<u8>) {
    let mut index = if spawned { spawn_index } else { 0 as usize };
    
    while &index < &bytes.len() {
        let byte = &bytes[index];
        match byte {
            0x00 => {
                // store
                let x = pop(&mut stack) as usize;
                let y = (pop(&mut stack) as usize) << 8;
                let value = pop(&mut stack);
                heap[x + y] = value;
            }
            0x01 => {
                // get
                let x = pop(&mut stack) as usize;
                let y = (pop(&mut stack) as usize) << 8;
                push(&mut stack, heap[x + y]);
            }
            0x02 => {
                // print
                println!("Stack: {:?}", stack);
            }
            0x03 => {
                // push
                push(&mut stack, bytes[index + 1]);
                index += 1;
            }
            0x04 => {
                // pop
                pop(&mut stack);
            }
            0x05 => {
                // write
                let x = pop(&mut stack) as usize;
                let mut y = pop(&mut stack) as usize;
                let color_index = pop(&mut stack);
                if y >= 144 {
                    y = y - 144;
                }
                pix_buffer[y][x] = color_index;
            }
            0x06 => {
                // add
                let a = pop(&mut stack) as u16;
                let b = pop(&mut stack) as u16;
                push(&mut stack, ((a + b) % 256) as u8);
            }
            0x07 => {
                // subtract
                let a = pop(&mut stack) as i16;
                let b = pop(&mut stack) as i16;
                let mut diff = b - a;
                if diff < 0 {
                    diff = 255 + diff + 1
                }
                push(&mut stack, diff as u8);
            }
            0x08 => {
                // multiply
                let a = pop(&mut stack) as u16;
                let b = pop(&mut stack) as u16;
                push(&mut stack, ((a * b) % 256) as u8);
            }
            0x09 => {
                // divide
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, (b / a) as u8);
            }
            0x0a => {
                // modulo
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, (b % a) as u8);
            }
            0x0b => {
                // routine
                let address = ((bytes[index + 1] as u16) << 8) + bytes[index + 2] as u16;
                index += 2;
                routines[address as usize] = (index + 1) as u32;
                while bytes[index] != 0x10 || bytes[index - 1] == 0x03 {
                    index += 1;
                }
            }
            0x0c => {
                // call
                let x = pop(&mut stack) as usize;
                let y = (pop(&mut stack) as usize) << 8;
                let start = routines[y + x];
                let evaled = bcode(
                    &bytes,
                    pix_buffer,
                    true,
                    start as usize,
                    heap,
                    stack.clone(),
                    routines.clone(),
                    tx,
                    &mut palette
                );
                stack = evaled.1;
                heap = evaled.2;
            }
            0x10 => {
                // end
                if spawned {
                    break;
                } else {
                    panic!("Unexpected End");
                }
            }
            0x0e => {
                // duplicate
                let last = stack.last().expect("Attempted to duplicate empty stack").to_owned();
                push(&mut stack, last);
            }
            0x0f => {
                // swap
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, a);
                push(&mut stack, b);
            }
            0x0d => {
                // copy
                let i = pop(&mut stack) as usize;
                let c = stack[i];
                push(&mut stack, c);
            }
            0x11 => {
                // if
                let condition = pop(&mut stack);
                let x = bytes[index + 2] as usize;
                let y = (bytes[index + 1] as usize) << 8;
                index = index + 2;
                if condition != 0 {
                    let start = routines[y + x];
                    let evaled = bcode(
                        &bytes,
                        pix_buffer,
                        true,
                        start as usize,
                        heap,
                        stack.clone(),
                        routines.clone(),
                        tx,
                        &mut palette
                    );
                    stack = evaled.1;
                    heap = evaled.2;
                }
            }
            0x12 => {
                // if
                let condition = pop(&mut stack);
                let x = bytes[index + 2] as usize;
                let y = (bytes[index + 1] as usize) << 8;
                index = index + 2;
                if condition == 0 {
                    let start = routines[y + x];
                    let evaled = bcode(
                        &bytes,
                        pix_buffer,
                        true,
                        start as usize,
                        heap,
                        stack.clone(),
                        routines.clone(),
                        tx,
                        &mut palette
                    );
                    stack = evaled.1;
                    heap = evaled.2;
                }
            }
            0x13 => {
                // greaterthan
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, if a > b { 1 } else { 0 });
            }
            0x14 => {
                // lessthan
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, if a < b { 1 } else { 0 });
            }
            0x15 => {
                // sprite
                let mut x = pop(&mut stack) as usize;
                let origx = x.clone();
                let mut y = pop(&mut stack) as usize;
                index += 1;
                while bytes[index] != 0xff {
                    if bytes[index] != 0xfe && bytes[index] != 0xee {
                        let color_index = bytes[index];
                        if heap[0xff0d] == 0 {
                            if y >= 144 {
                                y = y - 144;
                            }
                            if x >= 256 {
                                x = x - 256
                            }
                        }
                        pix_buffer[y][x] = color_index;
                        x += 1;
                        index += 1;
                    } else if bytes[index] == 0xfe {
                        x = origx;
                        y += 1;
                        index += 1;
                    } else {
                        x += 1;
                        index += 1;
                    }
                }
            }
            0x16 => {
                // bitnot
                let a = pop(&mut stack);
                push(&mut stack, !a);
            }
            0x17 => {
                // bitand
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, a & b);
            }
            0x18 => {
                // bitor
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, a | b);
            }
            0x19 => {
                // bitxor
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                push(&mut stack, a ^ b);
            }
            0x1a => {
                // random
                push(&mut stack, random::<u8>());
            }
            0x1b => {
                // repeat
                let x = bytes[index + 2] as usize;
                let y = (bytes[index + 1] as usize) << 8;
                let loops = pop(&mut stack);
                index = index + 2;
                let start = routines[y + x];
                for _a in 0..loops {
                    let evaled = bcode(
                        &bytes,
                        pix_buffer,
                        true,
                        start as usize,
                        heap,
                        stack.clone(),
                        routines.clone(),
                        tx,
                        &mut palette
                    );
                    stack = evaled.1;
                    heap = evaled.2;
                }
            }
            0x1c => {
                // beep
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                tx.send(format!("beep {} {}", a, b)).unwrap();
            }
            0x1d => {
                // empty
                tx.send("empty".to_string()).unwrap();
            }
            0x1e => {
                // boop
                let a = pop(&mut stack);
                let b = pop(&mut stack);
                tx.send(format!("boop {} {}", a, b)).unwrap();
            }
            0x1f => {
                // noise
                let a = pop(&mut stack);
                tx.send(format!("noise {}", a)).unwrap();
            }
            0x20 => {
                // color
                let color_index = bytes[index + 1];
                let r = bytes[index + 2];
                let g = bytes[index + 3];
                let b = bytes[index + 4];
                palette[color_index as usize] = [r,g,b,0xff];
                index += 4;
            }
            0x21 => {
                // line
                let y2 = pop(&mut stack) as isize;
                let x2 = pop(&mut stack) as isize;
                let y1 = pop(&mut stack) as isize;
                let x1 = pop(&mut stack) as isize;
                let color_index = pop(&mut stack);
                for (x, y) in Bresenham::new((x1, y1), (x2, y2)) {
                    pix_buffer[y as usize][x as usize] = color_index;
                }
            }
            0x22 => {
                // rectangle
                let height = pop(&mut stack) as isize;
                let width = pop(&mut stack) as isize;
                let y = pop(&mut stack) as isize;
                let x = pop(&mut stack) as isize;
                let color_index = pop(&mut stack);
                for ry in y..y+height {
                    for rx in x..x+width {
                        pix_buffer[ry as usize][rx as usize] = color_index;
                    }
                }
            }
            0x23 => {
                // text
                let x = pop(&mut stack) as usize;
                let y = pop(&mut stack) as usize;
                let color_index = pop(&mut stack);
                let mut text = Vec::with_capacity(256);
                index += 1;
                while bytes[index] != 0xff {
                    text.push(bytes[index]);
                    index += 1;
                }
                text::draw_text(&String::from_utf8_lossy(text.as_slice()).to_string(), &(x as u16), &(y as u16), color_index, pix_buffer);
            }
            _ => {
                panic!("Unknown Command {}", byte);
            }
        }
        index += 1;
    }
    return (routines, stack, heap);
}
