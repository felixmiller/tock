use rexpect::errors::Error;
use rexpect::session::PtySession;
use rexpect::spawn;

fn kill_qemu(p: &mut PtySession) -> Result<(), Error> {
    p.send_control('a')?;
    p.send("x")?;
    p.flush()?;

    Ok(())
}

fn hifive1() -> Result<(), Error> {
    let mut p = spawn("make qemu -C ../../boards/hifive1", Some(3_000))?;

    p.exp_string("HiFive1 initialization complete.")?;
    p.exp_string("Entering main loop.")?;

    // Test completed, kill QEMU
    kill_qemu(&mut p)?;

    p.exp_eof()?;
    Ok(())
}

fn opentitan() -> Result<(), Error> {
    let mut p = spawn(
        "make OPENTITAN_BOOT_ROM=../../opentitan-boot-rom.elf qemu -C ../../boards/opentitan",
        Some(10_000),
    )?;

    p.exp_string("Boot ROM initialisation has completed, jump into flash!")?;
    p.exp_string("Entering main loop")?;

    // Test completed, kill QEMU
    kill_qemu(&mut p)?;

    p.exp_eof()?;
    Ok(())
}

fn main() {
    hifive1().unwrap_or_else(|e| panic!("hifive1 job failed with {}", e));
    opentitan().unwrap_or_else(|e| panic!("opentitan job failed with {}", e));
}
