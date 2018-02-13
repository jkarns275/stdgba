#![feature(proc_macro, rustc_private)]
extern crate proc_macro;
extern crate syn;

extern crate syntax;
use syntax::parse::token;

use proc_macro::{ TokenStream, TokenNode };

extern crate image;

#[macro_use]
extern crate quote;
use quote::*;

use std::mem::transmute;

#[proc_macro]
pub fn img_as_palleted_sprite_4bpp(input: TokenStream) -> TokenStream {
    let mut colors = Vec::<u16>::with_capacity(1 << 4);
    colors.push(0);

    let img = load_img(input);
    let (width, height) = img.dimensions();

    if (width * height) & 1 == 1 {
        panic!("image must have an even number of pixels in order to convert it to 4bpp")
    }

    let mut pixels =  vec![0u8; (width * height) as usize >> 1];
    let mut index = 0usize;
    for iy in 0..height / 8 {
        for ix in 0..width / 8 {
            for y in 0..8 {
                //println!("");
                for x in 0..8 {
                    let rgb = img.get_pixel(ix * 8 + x, iy * 8 + y).data;
                    //print!("Pixel at ({}, {}): {:?}; ", ix * 8 + x, iy * 8 + y, rgb);
                    let converted_red = ((rgb[0] as f32 / 255.0f32) * 31.0f32) as u16;
                    let converted_green = ((rgb[1] as f32 / 255.0f32) * 31.0f32) as u16;
                    let converted_blue = ((rgb[2] as f32 / 255.0f32) * 31.0f32) as u16;

                    let color = (converted_blue << 10) | (converted_green << 5) | converted_red;
                    let color_index =
                        if let Some(color_index) = colors.iter().position(|&item| item == color) {
                            if color_index > 15 {
                                panic!("image contains more than 16 colors: a 4bpp image can only have 16 colors");
                            }
                            color_index
                        } else {
                            let color_index = colors.len();
                            if color_index == 16 {
                                panic!("image contains more than 16 colors: a 4bpp image can only have 16 colors");
                            }

                        colors.push(color);
                        color_index
                    };
                    if index & 1 == 0 {
                        pixels[index >> 1] |= color_index as u8;
                    } else {
                        pixels[index >> 1] |= (color_index << 4) as u8;
                    }
                    index += 1;
                }
            }

        }
    }

    /*
    let mut newln = 0;
    print!("palette: {{ ");
    for x in colors.iter() {
        if newln == 16 {
            newln = 0;
            println!("");
        }
        print!("{:x}, ", x);
        newln += 1;
    }
    newln = 0;
    println!("}}\nimg: ");
    for x in pixels.iter() {
        if newln == 32 {
            newln = 0;
            println!("");
        }
        print!("{:02x}, ", x);
        newln += 1;
    }
    println!("}}");
    */

    (quote! { (&[#(#colors),*], &[#(#pixels),*]) }).into()
}


#[proc_macro]
pub fn img_as_palleted_sprite_8bpp(input: TokenStream) -> TokenStream {
    let mut colors = Vec::<u16>::with_capacity(1 << 8);
    colors.push(0);

    let img = load_img(input);
    let (width, height) = img.dimensions();

    if (width * height) & 1 == 1 {
        panic!("image must have an even number of pixels in order to convert it to 4bpp")
    }



    let mut pixels = vec![0u8; (width * height) as usize];
    let mut index = 0usize;
    for iy in 0..height / 8 {
        for ix in 0..width / 8 {
            for y in 0..8 {
                //println!("");
                for x in 0..8 {
                    let rgb = img.get_pixel(ix * 8 + x, iy * 8 + y).data;
                    //print!("Pixel at ({}, {}): {:?}; ", ix * 8 + x, iy * 8 + y, rgb);
                    let converted_red = ((rgb[0] as f32 / 255.0f32) * 31.0f32) as u16;
                    let converted_green = ((rgb[1] as f32 / 255.0f32) * 31.0f32) as u16;
                    let converted_blue = ((rgb[2] as f32 / 255.0f32) * 31.0f32) as u16;

                    let color = (converted_blue << 10) | (converted_green << 5) | converted_red;
                    let color_index =
                        if let Some(color_index) = colors.iter().position(|&item| item == color) {
                            if color_index > (1 << 8) - 1 {
                                panic!("image contains more than 256 colors: a 8bpp image can only have 256 colors");
                            }
                            color_index
                        } else {
                            let color_index = colors.len();
                            if color_index == 1 << 8 {
                                panic!("image contains more than 256 colors: a 8bpp image can only have 256 colors");
                            }
                            colors.push(color);
                            color_index
                    };
                    pixels[index] |= color_index as u8;
                    index += 1;
                }
            }

        }
    }

    /*
    let mut newln = 0;
    print!("palette: {{ ");
    for x in colors.iter() {
        if newln == 16 {
            newln = 0;
            println!("");
        }
        print!("{:x}, ", x);
        newln += 1;
    }
    newln = 0;
    println!("}}\nimg: ");
    for x in pixels.iter() {
        if newln == 32 {
            newln = 0;
            println!("");
        }
        print!("{:02x}, ", x);
        newln += 1;
    }
    println!("}}");
    */
    let p = (quote! { (&[#(#colors),*], &[#(#pixels),*]) });
    p.into()
}

#[proc_macro]
pub fn img_as_palleted_sprite_16bpp(_input: TokenStream) -> TokenStream {
    panic!("aa")
}

fn load_img(input: TokenStream) -> image::RgbImage {
    let tokens: Vec<_> = input.into_iter().collect();
    if tokens.len() != 1 {
        panic!(format!("Argument should be a single string, but got {} arguments", tokens.len()));
    }

    let file_path = match tokens[0].kind {
        TokenNode::Literal(ref x) => {//proc_macro::Literal(token::Token::Literal(token::Lit::Str_(interned)))) => {
            let trans = unsafe { transmute::<proc_macro::Literal, token::Token>(x.clone()) };
            match trans {
                token::Token::Literal(token::Lit::Str_(s), _) => {
                    s.as_str().to_string()
                },
                token::Token::Literal(token::Lit::StrRaw(s, _), _) => {
                    s.as_str().to_string()
                },
                x @ _ => panic!(format!("Argument should be a string, got {:?}", x))
            }
        },
        ref x @ _ => panic!(format!("Argument should be a string, got {:?}", x))
    };

    let img_res = image::open(&file_path);
    let img = match img_res {
        Ok(i) => i.to_rgb(),
        _ => {
            panic!(format!("could not find image in the specified path '{}'", tokens.len()));
        }
    };

    img
}

