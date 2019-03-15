/****************************************************************************
    Copyleft (c) 2015 Osspial All Rights Reserved.

    This file is part of hidapi-rs, based on hidapi_rust by Roland Ruckerbauer.

    hidapi-rs is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    hidapi-rs is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MSRCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with hidapi-rs.  If not, see <http://www.gnu.org/licenses/>.
****************************************************************************/


//! Prints out a list of HID devices

extern crate hidapi;

use hidapi::HidApi;

fn main() {
    println!("Printing all available hid devices.");

    let api = HidApi::new().unwrap();

    for device in &api.devices() {
        println!("{:#?}", device);
    }
}