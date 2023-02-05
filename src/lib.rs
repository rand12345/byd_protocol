// PylonTech, BYD, and unknown interface based on SimpBMS
// Data references  https://github.com/Tom-evnut/BMWI3BMS/blob/master/BMWI3BMS.ino

use log::info;
use solax_can_bus::SolaxBms;
use std::time::Duration;

pub fn custom_frames(
    bmsdata: SolaxBms,
    charger_volts_high: Option<u16>,
    charger_volts_low: Option<u16>,
    timeout: Duration,
) -> anyhow::Result<Vec<(u16, Vec<u8>)>> {
    match bmsdata.timestamp {
        Some(time) => {
            if time.elapsed() < timeout {
                info!("Data is {:?} old", time.elapsed(),);
            } else {
                return Err(anyhow::anyhow!(
                    "Data is too old {:?}, timeout is {:?}",
                    time.elapsed(),
                    timeout
                ));
            }
        }
        None => return Err(anyhow::anyhow!("BMS timestamp is invalid")),
    }
    let charge_volts_high = match charger_volts_high {
        Some(val) => val.to_le_bytes(),
        None => 3936u16.to_le_bytes(), // 96 * 4.1v
    };

    let charge_volts_low = match charger_volts_low {
        Some(val) => val.to_le_bytes(),
        None => 2880u16.to_le_bytes(), // 96 * 3v
    };
    let charge_current = bmsdata.charge_max.to_le_bytes();
    let discharge_current = bmsdata.discharge_max.to_le_bytes();
    let soc = bmsdata.capacity.to_le_bytes();

    let pack_volts = bmsdata.voltage.to_le_bytes();
    let current = bmsdata.current.to_le_bytes();
    let temp = bmsdata.int_temp.to_le_bytes();

    let cap = (52000u16 / 65).to_le_bytes(); // capacity vs voltage
    let temp_high = bmsdata.cell_temp_max.to_le_bytes();
    let temp_low = bmsdata.cell_temp_min.to_le_bytes();

    Ok([
        (0x618, [0x0, b'B', b'Y', b'D', 0x0, 0x0, 0x0, 0x0].to_vec()),
        (0x5d8, [0x0, b'B', b'Y', b'D', 0x0, 0x0, 0x0, 0x0].to_vec()),
        (
            0x558,
            [0x3, 0x13, 0x0, 0x4, cap[1], cap[0], 0x5, 0x7].to_vec(),
        ),
        (0x598, [0x0, 0x0, 0x12, 0x34, 0x0, 0x0, 0x4, 0x4f].to_vec()),
        (
            0x358,
            [
                charge_volts_high[1],
                charge_volts_high[0],
                charge_volts_low[1],
                charge_volts_low[0],
                discharge_current[1],
                discharge_current[0],
                charge_current[1],
                charge_current[0],
            ]
            .to_vec(),
        ),
        (
            0x3d8,
            [soc[1], soc[0], soc[1], soc[0], 0x0, 0x0, 0xf9, 0x0].to_vec(),
        ), // 4 & 5 are ampsecond * 0.00277 ??
        (0x458, [0x0, 0x0, 0x12, 0x34, 0x0, 0x0, 0x56, 0x78].to_vec()),
        (
            0x518,
            [
                temp_high[1],
                temp_high[0],
                temp_low[1],
                temp_low[0],
                0xff,
                0xff,
                0xff,
                0xff,
            ]
            .to_vec(),
        ),
        (
            0x4d8,
            [
                pack_volts[1],
                pack_volts[0],
                current[1],
                current[0],
                temp[1],
                temp[0],
                0x3,
                0x8,
            ]
            .to_vec(),
        ),
        (0x158, [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0].to_vec()),
    ]
    .to_vec())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
