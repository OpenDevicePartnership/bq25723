/// Root block of the Device driver
#[derive(Debug)]
pub struct Device<I> {
    pub(crate) interface: I,
    #[doc(hidden)]
    base_address: u8,
}
impl<I> Device<I> {
    /// Create a new instance of the block based on device interface
    pub const fn new(interface: I) -> Self {
        Self {
            interface,
            base_address: 0,
        }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        &mut self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.charge_option_0().read()?;
        callback(0 + 0 * 0, "charge_option_0", reg.into());
        let reg = self.charge_current().read()?;
        callback(2 + 0 * 0, "charge_current", reg.into());
        let reg = self.charge_voltage().read()?;
        callback(4 + 0 * 0, "charge_voltage", reg.into());
        let reg = self.otg_voltage().read()?;
        callback(6 + 0 * 0, "otg_voltage", reg.into());
        let reg = self.otg_current().read()?;
        callback(8 + 0 * 0, "otg_current", reg.into());
        let reg = self.input_voltage().read()?;
        callback(10 + 0 * 0, "input_voltage", reg.into());
        let reg = self.vsys_min().read()?;
        callback(12 + 0 * 0, "vsys_min", reg.into());
        let reg = self.iin_host().read()?;
        callback(14 + 0 * 0, "iin_host", reg.into());
        let reg = self.charger_status().read()?;
        callback(32 + 0 * 0, "charger_status", reg.into());
        let reg = self.prochot_status().read()?;
        callback(34 + 0 * 0, "prochot_status", reg.into());
        let reg = self.iin_dpm().read()?;
        callback(36 + 0 * 0, "iin_dpm", reg.into());
        let reg = self.adc_vbus_psys().read()?;
        callback(38 + 0 * 0, "adc_vbus_psys", reg.into());
        let reg = self.adc_ibat().read()?;
        callback(40 + 0 * 0, "adc_ibat", reg.into());
        let reg = self.adc_iin_cmpin().read()?;
        callback(42 + 0 * 0, "adc_iin_cmpin", reg.into());
        let reg = self.adc_vsys_vbat().read()?;
        callback(44 + 0 * 0, "adc_vsys_vbat", reg.into());
        let reg = self.manufacturer_id().read()?;
        callback(46 + 0 * 0, "manufacturer_id", reg.into());
        let reg = self.device_id().read()?;
        callback(47 + 0 * 0, "device_id", reg.into());
        let reg = self.charge_option_1().read()?;
        callback(48 + 0 * 0, "charge_option_1", reg.into());
        let reg = self.charge_option_2().read()?;
        callback(50 + 0 * 0, "charge_option_2", reg.into());
        let reg = self.charge_option_3().read()?;
        callback(52 + 0 * 0, "charge_option_3", reg.into());
        let reg = self.prochot_option_0().read()?;
        callback(54 + 0 * 0, "prochot_option_0", reg.into());
        let reg = self.prochot_option_1().read()?;
        callback(56 + 0 * 0, "prochot_option_1", reg.into());
        let reg = self.adc_option().read()?;
        callback(58 + 0 * 0, "adc_option", reg.into());
        let reg = self.charge_option_4().read()?;
        callback(60 + 0 * 0, "charge_option_4", reg.into());
        let reg = self.vmin_active_protection().read()?;
        callback(62 + 0 * 0, "vmin_active_protection", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.charge_option_0().read_async().await?;
        callback(0 + 0 * 0, "charge_option_0", reg.into());
        let reg = self.charge_current().read_async().await?;
        callback(2 + 0 * 0, "charge_current", reg.into());
        let reg = self.charge_voltage().read_async().await?;
        callback(4 + 0 * 0, "charge_voltage", reg.into());
        let reg = self.otg_voltage().read_async().await?;
        callback(6 + 0 * 0, "otg_voltage", reg.into());
        let reg = self.otg_current().read_async().await?;
        callback(8 + 0 * 0, "otg_current", reg.into());
        let reg = self.input_voltage().read_async().await?;
        callback(10 + 0 * 0, "input_voltage", reg.into());
        let reg = self.vsys_min().read_async().await?;
        callback(12 + 0 * 0, "vsys_min", reg.into());
        let reg = self.iin_host().read_async().await?;
        callback(14 + 0 * 0, "iin_host", reg.into());
        let reg = self.charger_status().read_async().await?;
        callback(32 + 0 * 0, "charger_status", reg.into());
        let reg = self.prochot_status().read_async().await?;
        callback(34 + 0 * 0, "prochot_status", reg.into());
        let reg = self.iin_dpm().read_async().await?;
        callback(36 + 0 * 0, "iin_dpm", reg.into());
        let reg = self.adc_vbus_psys().read_async().await?;
        callback(38 + 0 * 0, "adc_vbus_psys", reg.into());
        let reg = self.adc_ibat().read_async().await?;
        callback(40 + 0 * 0, "adc_ibat", reg.into());
        let reg = self.adc_iin_cmpin().read_async().await?;
        callback(42 + 0 * 0, "adc_iin_cmpin", reg.into());
        let reg = self.adc_vsys_vbat().read_async().await?;
        callback(44 + 0 * 0, "adc_vsys_vbat", reg.into());
        let reg = self.manufacturer_id().read_async().await?;
        callback(46 + 0 * 0, "manufacturer_id", reg.into());
        let reg = self.device_id().read_async().await?;
        callback(47 + 0 * 0, "device_id", reg.into());
        let reg = self.charge_option_1().read_async().await?;
        callback(48 + 0 * 0, "charge_option_1", reg.into());
        let reg = self.charge_option_2().read_async().await?;
        callback(50 + 0 * 0, "charge_option_2", reg.into());
        let reg = self.charge_option_3().read_async().await?;
        callback(52 + 0 * 0, "charge_option_3", reg.into());
        let reg = self.prochot_option_0().read_async().await?;
        callback(54 + 0 * 0, "prochot_option_0", reg.into());
        let reg = self.prochot_option_1().read_async().await?;
        callback(56 + 0 * 0, "prochot_option_1", reg.into());
        let reg = self.adc_option().read_async().await?;
        callback(58 + 0 * 0, "adc_option", reg.into());
        let reg = self.charge_option_4().read_async().await?;
        callback(60 + 0 * 0, "charge_option_4", reg.into());
        let reg = self.vmin_active_protection().read_async().await?;
        callback(62 + 0 * 0, "vmin_active_protection", reg.into());
        Ok(())
    }
    pub fn charge_option_0(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption0, ::device_driver::RW> {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption0, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption0::new,
        )
    }
    pub fn charge_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeCurrent, ::device_driver::RW> {
        let address = self.base_address + 2;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeCurrent, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeCurrent::new,
        )
    }
    pub fn charge_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeVoltage, ::device_driver::RW> {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeVoltage, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeVoltage::new,
        )
    }
    pub fn otg_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::OtgVoltage, ::device_driver::RW> {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::OtgVoltage, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::OtgVoltage::new,
        )
    }
    pub fn otg_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::OtgCurrent, ::device_driver::RW> {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::OtgCurrent, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::OtgCurrent::new,
        )
    }
    pub fn input_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::InputVoltage, ::device_driver::RW> {
        let address = self.base_address + 10;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::InputVoltage, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::InputVoltage::new,
        )
    }
    pub fn vsys_min(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::VsysMin, ::device_driver::RW> {
        let address = self.base_address + 12;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::VsysMin, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::VsysMin::new,
        )
    }
    pub fn iin_host(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::IinHost, ::device_driver::RW> {
        let address = self.base_address + 14;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::IinHost, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::IinHost::new,
        )
    }
    pub fn charger_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargerStatus, ::device_driver::RW> {
        let address = self.base_address + 32;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargerStatus, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargerStatus::new,
        )
    }
    pub fn prochot_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotStatus, ::device_driver::RW> {
        let address = self.base_address + 34;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotStatus, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotStatus::new,
        )
    }
    pub fn iin_dpm(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::IinDpm, ::device_driver::RO> {
        let address = self.base_address + 36;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::IinDpm, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::IinDpm::new,
        )
    }
    pub fn adc_vbus_psys(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcVbusPsys, ::device_driver::RO> {
        let address = self.base_address + 38;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcVbusPsys, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcVbusPsys::new,
        )
    }
    pub fn adc_ibat(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcIbat, ::device_driver::RO> {
        let address = self.base_address + 40;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcIbat, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcIbat::new,
        )
    }
    pub fn adc_iin_cmpin(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcIinCmpin, ::device_driver::RO> {
        let address = self.base_address + 42;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcIinCmpin, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcIinCmpin::new,
        )
    }
    pub fn adc_vsys_vbat(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcVsysVbat, ::device_driver::RO> {
        let address = self.base_address + 44;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcVsysVbat, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcVsysVbat::new,
        )
    }
    pub fn manufacturer_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ManufacturerId, ::device_driver::RO> {
        let address = self.base_address + 46;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ManufacturerId, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ManufacturerId::new,
        )
    }
    pub fn device_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::DeviceId, ::device_driver::RO> {
        let address = self.base_address + 47;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::DeviceId, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::DeviceId::new,
        )
    }
    pub fn charge_option_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption1, ::device_driver::RW> {
        let address = self.base_address + 48;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption1, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption1::new,
        )
    }
    pub fn charge_option_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption2, ::device_driver::RW> {
        let address = self.base_address + 50;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption2, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption2::new,
        )
    }
    pub fn charge_option_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption3, ::device_driver::RW> {
        let address = self.base_address + 52;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption3, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption3::new,
        )
    }
    pub fn prochot_option_0(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotOption0, ::device_driver::RW> {
        let address = self.base_address + 54;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotOption0, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotOption0::new,
        )
    }
    pub fn prochot_option_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotOption1, ::device_driver::RW> {
        let address = self.base_address + 56;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotOption1, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotOption1::new,
        )
    }
    pub fn adc_option(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcOption, ::device_driver::RW> {
        let address = self.base_address + 58;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcOption, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AdcOption::new,
        )
    }
    pub fn charge_option_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption4, ::device_driver::RW> {
        let address = self.base_address + 60;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption4, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption4::new,
        )
    }
    pub fn vmin_active_protection(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::VminActiveProtection, ::device_driver::RW> {
        let address = self.base_address + 62;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::VminActiveProtection, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::VminActiveProtection::new,
        )
    }
}
/// Module containing the generated fieldsets of the registers and commands
pub mod field_sets {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption0 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargeOption0 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption0 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [14, 231] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `chrg_inhibit` field of the register.
        ///
        /// Charge Inhibit. When this bit is 0, battery charging will start with valid values in the ChargeVoltage() and ChargeCurrent() registers.
        pub fn chrg_inhibit(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_iin_dpm` field of the register.
        ///
        /// IIN_DPM Enable. Host writes this bit to enable IIN_DPM regulation loop. When the IIN_DPM is disabled by the charger (refer to IIN_DPM_AUTO_DISABLE), this bit goes LOW.
        pub fn en_iin_dpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `en_ldo` field of the register.
        ///
        /// LDO Mode Enable. When battery voltage is below minimum system voltage, the charger is in pre-charge with LDO mode enabled. When enabled, precharge current is set by ChargeCurrent register and clamped below 384 mA (2-4 cell, 1 cell VBAT<3.0V) or 2A (1 cell 3.0V<VBAT<3.6V).
        pub fn en_ldo(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `ibat_gain` field of the register.
        ///
        /// IBAT Amplifier Ratio. The ratio of voltage on IBAT and voltage across SRP and SRN. 0=8x, 1=16x.
        pub fn ibat_gain(&self) -> super::IbatGain {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `iadpt_gain` field of the register.
        ///
        /// IADPT Amplifier Ratio. The ratio of voltage on IADPT and voltage across ACP and ACN. 0=20x, 1=40x.
        pub fn iadpt_gain(&self) -> super::IadptGain {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_learn` field of the register.
        ///
        /// LEARN mode allows the battery to discharge and converter to shut off while the adapter is present. It calibrates the battery gas gauge over a complete discharge/charge cycle.
        pub fn en_learn(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `vsys_uvp_enz` field of the register.
        ///
        /// Disable system under voltage protection. 0=VSYS UVP enabled (default), 1=VSYS UVP disabled.
        pub fn vsys_uvp_enz(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_cmp_latch` field of the register.
        ///
        /// Latch the independent comparator output after it is triggered at low state. If enabled in PROCHOT profile PP_CMP=1, STAT_COMP bit keeps 1 after triggered until read by host and cleared. Host can clear CMPOUT pin by toggling this bit.
        pub fn en_cmp_latch(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `dis_strgrv` field of the register.
        ///
        /// Switching HS MOSFET turn on gate drive strength. 0=Enable strong drive, 1=Disable strong drive.
        pub fn dis_strgrv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `pwm_freq` field of the register.
        ///
        /// Switching Frequency Selection. Recommend 1200 kHz with 1 uH, 800 kHz with 2.2 uH. 0=1200 kHz, 1=800 kHz.
        pub fn pwm_freq(&self) -> super::SwitchingFreq {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_ooa` field of the register.
        ///
        /// Out-of-Audio Enable. Set minimum PFM burst frequency to above 25 kHz to avoid audio noise.
        pub fn en_ooa(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `otg_on_chrgok` field of the register.
        ///
        /// Add OTG to CHRG_OK. Drive CHRG_OK HIGH when the device is in OTG mode.
        pub fn otg_on_chrgok(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `iin_dpm_auto_disable` field of the register.
        ///
        /// IIN_DPM Auto Disable. When CELL_BATPRESZ pin is LOW, the charger automatically disables the IIN_DPM function by setting EN_IIN_DPM to 0.
        pub fn iin_dpm_auto_disable(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `wdtmr_adj` field of the register.
        ///
        /// WATCHDOG Timer Adjust. Set maximum delay between consecutive host write of charge voltage or charge current command. If device does not receive a write within this period, ChargeCurrent is set to 0 mA.
        pub fn wdtmr_adj(&self) -> super::WatchdogTimerAdj {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 15) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_lwpwr` field of the register.
        ///
        /// Low Power Mode Enable. In low power mode, lowest quiescent current is achieved when only battery exists. REGN is off. PROCHOT, discharge current monitor buffer, power monitor buffer and independent comparator are disabled. ADC is not available.
        pub fn en_lwpwr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Write the `chrg_inhibit` field of the register.
        ///
        /// Charge Inhibit. When this bit is 0, battery charging will start with valid values in the ChargeVoltage() and ChargeCurrent() registers.
        pub fn set_chrg_inhibit(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_iin_dpm` field of the register.
        ///
        /// IIN_DPM Enable. Host writes this bit to enable IIN_DPM regulation loop. When the IIN_DPM is disabled by the charger (refer to IIN_DPM_AUTO_DISABLE), this bit goes LOW.
        pub fn set_en_iin_dpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_ldo` field of the register.
        ///
        /// LDO Mode Enable. When battery voltage is below minimum system voltage, the charger is in pre-charge with LDO mode enabled. When enabled, precharge current is set by ChargeCurrent register and clamped below 384 mA (2-4 cell, 1 cell VBAT<3.0V) or 2A (1 cell 3.0V<VBAT<3.6V).
        pub fn set_en_ldo(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `ibat_gain` field of the register.
        ///
        /// IBAT Amplifier Ratio. The ratio of voltage on IBAT and voltage across SRP and SRN. 0=8x, 1=16x.
        pub fn set_ibat_gain(&mut self, value: super::IbatGain) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `iadpt_gain` field of the register.
        ///
        /// IADPT Amplifier Ratio. The ratio of voltage on IADPT and voltage across ACP and ACN. 0=20x, 1=40x.
        pub fn set_iadpt_gain(&mut self, value: super::IadptGain) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `en_learn` field of the register.
        ///
        /// LEARN mode allows the battery to discharge and converter to shut off while the adapter is present. It calibrates the battery gas gauge over a complete discharge/charge cycle.
        pub fn set_en_learn(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `vsys_uvp_enz` field of the register.
        ///
        /// Disable system under voltage protection. 0=VSYS UVP enabled (default), 1=VSYS UVP disabled.
        pub fn set_vsys_uvp_enz(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_cmp_latch` field of the register.
        ///
        /// Latch the independent comparator output after it is triggered at low state. If enabled in PROCHOT profile PP_CMP=1, STAT_COMP bit keeps 1 after triggered until read by host and cleared. Host can clear CMPOUT pin by toggling this bit.
        pub fn set_en_cmp_latch(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `dis_strgrv` field of the register.
        ///
        /// Switching HS MOSFET turn on gate drive strength. 0=Enable strong drive, 1=Disable strong drive.
        pub fn set_dis_strgrv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `pwm_freq` field of the register.
        ///
        /// Switching Frequency Selection. Recommend 1200 kHz with 1 uH, 800 kHz with 2.2 uH. 0=1200 kHz, 1=800 kHz.
        pub fn set_pwm_freq(&mut self, value: super::SwitchingFreq) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `en_ooa` field of the register.
        ///
        /// Out-of-Audio Enable. Set minimum PFM burst frequency to above 25 kHz to avoid audio noise.
        pub fn set_en_ooa(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `otg_on_chrgok` field of the register.
        ///
        /// Add OTG to CHRG_OK. Drive CHRG_OK HIGH when the device is in OTG mode.
        pub fn set_otg_on_chrgok(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `iin_dpm_auto_disable` field of the register.
        ///
        /// IIN_DPM Auto Disable. When CELL_BATPRESZ pin is LOW, the charger automatically disables the IIN_DPM function by setting EN_IIN_DPM to 0.
        pub fn set_iin_dpm_auto_disable(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 13, &mut self.bits) };
        }
        ///Write the `wdtmr_adj` field of the register.
        ///
        /// WATCHDOG Timer Adjust. Set maximum delay between consecutive host write of charge voltage or charge current command. If device does not receive a write within this period, ChargeCurrent is set to 0 mA.
        pub fn set_wdtmr_adj(&mut self, value: super::WatchdogTimerAdj) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 13, 15, &mut self.bits) };
        }
        ///Write the `en_lwpwr` field of the register.
        ///
        /// Low Power Mode Enable. In low power mode, lowest quiescent current is achieved when only battery exists. REGN is off. PROCHOT, discharge current monitor buffer, power monitor buffer and independent comparator are disabled. ADC is not available.
        pub fn set_en_lwpwr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ChargeOption0 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption0> for [u8; 2] {
        fn from(val: ChargeOption0) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption0 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption0");
            d.field("chrg_inhibit", &self.chrg_inhibit());
            d.field("en_iin_dpm", &self.en_iin_dpm());
            d.field("en_ldo", &self.en_ldo());
            d.field("ibat_gain", &self.ibat_gain());
            d.field("iadpt_gain", &self.iadpt_gain());
            d.field("en_learn", &self.en_learn());
            d.field("vsys_uvp_enz", &self.vsys_uvp_enz());
            d.field("en_cmp_latch", &self.en_cmp_latch());
            d.field("dis_strgrv", &self.dis_strgrv());
            d.field("pwm_freq", &self.pwm_freq());
            d.field("en_ooa", &self.en_ooa());
            d.field("otg_on_chrgok", &self.otg_on_chrgok());
            d.field("iin_dpm_auto_disable", &self.iin_dpm_auto_disable());
            d.field("wdtmr_adj", &self.wdtmr_adj());
            d.field("en_lwpwr", &self.en_lwpwr());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption0 {{ ");
            defmt::write!(f, "chrg_inhibit: {=bool}, ", &self.chrg_inhibit());
            defmt::write!(f, "en_iin_dpm: {=bool}, ", &self.en_iin_dpm());
            defmt::write!(f, "en_ldo: {=bool}, ", &self.en_ldo());
            defmt::write!(f, "ibat_gain: {}, ", &self.ibat_gain());
            defmt::write!(f, "iadpt_gain: {}, ", &self.iadpt_gain());
            defmt::write!(f, "en_learn: {=bool}, ", &self.en_learn());
            defmt::write!(f, "vsys_uvp_enz: {=bool}, ", &self.vsys_uvp_enz());
            defmt::write!(f, "en_cmp_latch: {=bool}, ", &self.en_cmp_latch());
            defmt::write!(f, "dis_strgrv: {=bool}, ", &self.dis_strgrv());
            defmt::write!(f, "pwm_freq: {}, ", &self.pwm_freq());
            defmt::write!(f, "en_ooa: {=bool}, ", &self.en_ooa());
            defmt::write!(f, "otg_on_chrgok: {=bool}, ", &self.otg_on_chrgok());
            defmt::write!(f, "iin_dpm_auto_disable: {=bool}, ", &self.iin_dpm_auto_disable());
            defmt::write!(f, "wdtmr_adj: {}, ", &self.wdtmr_adj());
            defmt::write!(f, "en_lwpwr: {=bool}, ", &self.en_lwpwr());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption0 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption0 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption0 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption0 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption0 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption0 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption0 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeCurrent {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargeCurrent {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeCurrent {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `charge_current` field of the register.
        ///
        /// 7-bit charge current setting. With 10 mOhm sense resistor (RSNS_RSR=0), LSB=64 mA, range 0-8128 mA. With 5 mOhm sense resistor (RSNS_RSR=1), LSB=128 mA, range 0-16256 mA.
        pub fn charge_current(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 13) };
            raw
        }
        ///Write the `charge_current` field of the register.
        ///
        /// 7-bit charge current setting. With 10 mOhm sense resistor (RSNS_RSR=0), LSB=64 mA, range 0-8128 mA. With 5 mOhm sense resistor (RSNS_RSR=1), LSB=128 mA, range 0-16256 mA.
        pub fn set_charge_current(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 13, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ChargeCurrent {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeCurrent> for [u8; 2] {
        fn from(val: ChargeCurrent) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeCurrent {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeCurrent");
            d.field("charge_current", &self.charge_current());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeCurrent {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeCurrent {{ ");
            defmt::write!(f, "charge_current: {=u8}, ", &self.charge_current());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeCurrent {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeCurrent {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeCurrent {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeCurrent {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeCurrent {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeCurrent {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeCurrent {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeVoltage {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargeVoltage {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeVoltage {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `charge_voltage` field of the register.
        ///
        /// 12-bit charge voltage setting. LSB=8 mV, range 1024-19200 mV. Default depends on CELL_BATPRESZ pin setting (1S=4200 mV, 2S=8400 mV, 3S=12600 mV, 4S=16800 mV). Any write below 1024 mV or above 19200 mV is ignored.
        pub fn charge_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 3, 15) };
            raw
        }
        ///Write the `charge_voltage` field of the register.
        ///
        /// 12-bit charge voltage setting. LSB=8 mV, range 1024-19200 mV. Default depends on CELL_BATPRESZ pin setting (1S=4200 mV, 2S=8400 mV, 3S=12600 mV, 4S=16800 mV). Any write below 1024 mV or above 19200 mV is ignored.
        pub fn set_charge_voltage(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 3, 15, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ChargeVoltage {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeVoltage> for [u8; 2] {
        fn from(val: ChargeVoltage) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeVoltage {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeVoltage");
            d.field("charge_voltage", &self.charge_voltage());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeVoltage {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeVoltage {{ ");
            defmt::write!(f, "charge_voltage: {=u16}, ", &self.charge_voltage());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeVoltage {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeVoltage {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeVoltage {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeVoltage {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeVoltage {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeVoltage {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeVoltage {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtgVoltage {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for OtgVoltage {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl OtgVoltage {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [196, 9] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `otg_voltage` field of the register.
        ///
        /// 12-bit OTG output voltage setting. LSB=8 mV, range 3000-24000 mV. Default 5000 mV (USB). Any write below 3000 mV or above 24000 mV is ignored.
        pub fn otg_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 2, 14) };
            raw
        }
        ///Write the `otg_voltage` field of the register.
        ///
        /// 12-bit OTG output voltage setting. LSB=8 mV, range 3000-24000 mV. Default 5000 mV (USB). Any write below 3000 mV or above 24000 mV is ignored.
        pub fn set_otg_voltage(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 2, 14, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for OtgVoltage {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<OtgVoltage> for [u8; 2] {
        fn from(val: OtgVoltage) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for OtgVoltage {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("OtgVoltage");
            d.field("otg_voltage", &self.otg_voltage());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for OtgVoltage {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OtgVoltage {{ ");
            defmt::write!(f, "otg_voltage: {=u16}, ", &self.otg_voltage());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for OtgVoltage {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for OtgVoltage {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for OtgVoltage {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for OtgVoltage {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for OtgVoltage {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for OtgVoltage {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for OtgVoltage {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtgCurrent {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for OtgCurrent {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl OtgCurrent {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 60] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `otg_current` field of the register.
        ///
        /// 7-bit OTG output current limit. With 10 mOhm sense resistor, LSB=50 mA, range 0-6350 mA. Default 3000 mA.
        pub fn otg_current(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 15) };
            raw
        }
        ///Write the `otg_current` field of the register.
        ///
        /// 7-bit OTG output current limit. With 10 mOhm sense resistor, LSB=50 mA, range 0-6350 mA. Default 3000 mA.
        pub fn set_otg_current(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 15, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for OtgCurrent {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<OtgCurrent> for [u8; 2] {
        fn from(val: OtgCurrent) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for OtgCurrent {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("OtgCurrent");
            d.field("otg_current", &self.otg_current());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for OtgCurrent {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OtgCurrent {{ ");
            defmt::write!(f, "otg_current: {=u8}, ", &self.otg_current());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for OtgCurrent {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for OtgCurrent {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for OtgCurrent {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for OtgCurrent {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for OtgCurrent {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for OtgCurrent {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for OtgCurrent {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InputVoltage {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for InputVoltage {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl InputVoltage {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `input_voltage` field of the register.
        ///
        /// 8-bit input voltage (VINDPM) setting. LSB=64 mV, range 3200-19520 mV. Fixed DC offset of 3200 mV. Default is VBUS at no load minus 1.28 V.
        pub fn input_voltage(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 14) };
            raw
        }
        ///Write the `input_voltage` field of the register.
        ///
        /// 8-bit input voltage (VINDPM) setting. LSB=64 mV, range 3200-19520 mV. Fixed DC offset of 3200 mV. Default is VBUS at no load minus 1.28 V.
        pub fn set_input_voltage(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 14, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for InputVoltage {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<InputVoltage> for [u8; 2] {
        fn from(val: InputVoltage) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for InputVoltage {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("InputVoltage");
            d.field("input_voltage", &self.input_voltage());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for InputVoltage {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "InputVoltage {{ ");
            defmt::write!(f, "input_voltage: {=u8}, ", &self.input_voltage());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for InputVoltage {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for InputVoltage {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for InputVoltage {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for InputVoltage {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for InputVoltage {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for InputVoltage {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for InputVoltage {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VsysMin {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for VsysMin {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl VsysMin {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `vsys_min` field of the register.
        ///
        /// 8-bit minimum system voltage setting. LSB=100 mV, range 1000-19200 mV. Default depends on CELL_BATPRESZ pin (1S=3600 mV, 2S=6600 mV, 3S=9200 mV, 4S=12300 mV). Writing 0 sets to default based on CELL_BATPRESZ pin.
        pub fn vsys_min(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 16) };
            raw
        }
        ///Write the `vsys_min` field of the register.
        ///
        /// 8-bit minimum system voltage setting. LSB=100 mV, range 1000-19200 mV. Default depends on CELL_BATPRESZ pin (1S=3600 mV, 2S=6600 mV, 3S=9200 mV, 4S=12300 mV). Writing 0 sets to default based on CELL_BATPRESZ pin.
        pub fn set_vsys_min(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for VsysMin {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<VsysMin> for [u8; 2] {
        fn from(val: VsysMin) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VsysMin {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VsysMin");
            d.field("vsys_min", &self.vsys_min());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for VsysMin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VsysMin {{ ");
            defmt::write!(f, "vsys_min: {=u8}, ", &self.vsys_min());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for VsysMin {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VsysMin {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VsysMin {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VsysMin {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VsysMin {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VsysMin {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VsysMin {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IinHost {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for IinHost {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl IinHost {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 65] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `iin_host` field of the register.
        ///
        /// 7-bit input current limit set by host. With 10 mOhm sense resistor (RSNS_RAC=0), LSB=50 mA, range 50-6350 mA, default 3250 mA. 50 mA offset at code 0. With 5 mOhm sense resistor (RSNS_RAC=1), 100 mA offset at code 0. Upon adapter removal, reset to default.
        pub fn iin_host(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 15) };
            raw
        }
        ///Write the `iin_host` field of the register.
        ///
        /// 7-bit input current limit set by host. With 10 mOhm sense resistor (RSNS_RAC=0), LSB=50 mA, range 50-6350 mA, default 3250 mA. 50 mA offset at code 0. With 5 mOhm sense resistor (RSNS_RAC=1), 100 mA offset at code 0. Upon adapter removal, reset to default.
        pub fn set_iin_host(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 15, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for IinHost {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<IinHost> for [u8; 2] {
        fn from(val: IinHost) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IinHost {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IinHost");
            d.field("iin_host", &self.iin_host());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for IinHost {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IinHost {{ ");
            defmt::write!(f, "iin_host: {=u8}, ", &self.iin_host());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for IinHost {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IinHost {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IinHost {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IinHost {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IinHost {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IinHost {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IinHost {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargerStatus {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargerStatus {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargerStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `fault_otg_uvp` field of the register.
        ///
        /// OTG UVP fault. Latched until read by host.
        pub fn fault_otg_uvp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `fault_otg_ovp` field of the register.
        ///
        /// OTG OVP fault. Latched until read by host.
        pub fn fault_otg_ovp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `fault_force_converter_off` field of the register.
        ///
        /// Force converter off fault (when FORCE_CONV_OFF=1). Latched until read by host.
        pub fn fault_force_converter_off(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `fault_vsys_uvp` field of the register.
        ///
        /// VSYS_UVP fault status and clear. Latched until cleared by host writing this bit to 0. Set when system voltage is lower than VSYS_UVP and 7 restart attempts fail.
        pub fn fault_vsys_uvp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `fault_sysovp` field of the register.
        ///
        /// SYSOVP Status and Clear. When SYSOVP occurs, this bit is HIGH and converter is disabled. Write 0 or unplug adapter to clear the SYSOVP latch and re-enable converter.
        pub fn fault_sysovp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `fault_acoc` field of the register.
        ///
        /// ACOC fault. Latched until read by host.
        pub fn fault_acoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `fault_batoc` field of the register.
        ///
        /// BATOC fault. Latched until read by host. In PTM mode when EN_BATOC=1, indicates both BATOVP and BATOC.
        pub fn fault_batoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `fault_acov` field of the register.
        ///
        /// ACOV fault. Latched until read by host.
        pub fn fault_acov(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `in_otg` field of the register.
        ///
        /// Charger is in OTG mode.
        pub fn in_otg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `in_pchrg` field of the register.
        ///
        /// Charger is in pre-charge.
        pub fn in_pchrg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `in_fchrg` field of the register.
        ///
        /// Charger is in fast charge.
        pub fn in_fchrg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `in_iin_dpm` field of the register.
        ///
        /// Charger is in IIN_DPM during forward mode.
        pub fn in_iin_dpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `in_vindpm` field of the register.
        ///
        /// Charger is in VINDPM during forward mode, or voltage regulation during OTG mode.
        pub fn in_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `in_vap` field of the register.
        ///
        /// Charger is operated in VAP mode.
        pub fn in_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `ico_done` field of the register.
        ///
        /// ICO routine successfully completed.
        pub fn ico_done(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `stat_ac` field of the register.
        ///
        /// Input source status. Valid when VBUS is within 3.5 V to 26 V range. Different from CHRG_OK; CHRG_OK valid implies STAT_AC valid but not vice versa. Force converter off, ACOC, TSHUT, SYSOVP, VSYS_UVP, BATOVP can pull CHRG_OK low.
        pub fn stat_ac(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Write the `fault_vsys_uvp` field of the register.
        ///
        /// VSYS_UVP fault status and clear. Latched until cleared by host writing this bit to 0. Set when system voltage is lower than VSYS_UVP and 7 restart attempts fail.
        pub fn set_fault_vsys_uvp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `fault_sysovp` field of the register.
        ///
        /// SYSOVP Status and Clear. When SYSOVP occurs, this bit is HIGH and converter is disabled. Write 0 or unplug adapter to clear the SYSOVP latch and re-enable converter.
        pub fn set_fault_sysovp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ChargerStatus {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargerStatus> for [u8; 2] {
        fn from(val: ChargerStatus) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargerStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargerStatus");
            d.field("fault_otg_uvp", &self.fault_otg_uvp());
            d.field("fault_otg_ovp", &self.fault_otg_ovp());
            d.field("fault_force_converter_off", &self.fault_force_converter_off());
            d.field("fault_vsys_uvp", &self.fault_vsys_uvp());
            d.field("fault_sysovp", &self.fault_sysovp());
            d.field("fault_acoc", &self.fault_acoc());
            d.field("fault_batoc", &self.fault_batoc());
            d.field("fault_acov", &self.fault_acov());
            d.field("in_otg", &self.in_otg());
            d.field("in_pchrg", &self.in_pchrg());
            d.field("in_fchrg", &self.in_fchrg());
            d.field("in_iin_dpm", &self.in_iin_dpm());
            d.field("in_vindpm", &self.in_vindpm());
            d.field("in_vap", &self.in_vap());
            d.field("ico_done", &self.ico_done());
            d.field("stat_ac", &self.stat_ac());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargerStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargerStatus {{ ");
            defmt::write!(f, "fault_otg_uvp: {=bool}, ", &self.fault_otg_uvp());
            defmt::write!(f, "fault_otg_ovp: {=bool}, ", &self.fault_otg_ovp());
            defmt::write!(
                f,
                "fault_force_converter_off: {=bool}, ",
                &self.fault_force_converter_off()
            );
            defmt::write!(f, "fault_vsys_uvp: {=bool}, ", &self.fault_vsys_uvp());
            defmt::write!(f, "fault_sysovp: {=bool}, ", &self.fault_sysovp());
            defmt::write!(f, "fault_acoc: {=bool}, ", &self.fault_acoc());
            defmt::write!(f, "fault_batoc: {=bool}, ", &self.fault_batoc());
            defmt::write!(f, "fault_acov: {=bool}, ", &self.fault_acov());
            defmt::write!(f, "in_otg: {=bool}, ", &self.in_otg());
            defmt::write!(f, "in_pchrg: {=bool}, ", &self.in_pchrg());
            defmt::write!(f, "in_fchrg: {=bool}, ", &self.in_fchrg());
            defmt::write!(f, "in_iin_dpm: {=bool}, ", &self.in_iin_dpm());
            defmt::write!(f, "in_vindpm: {=bool}, ", &self.in_vindpm());
            defmt::write!(f, "in_vap: {=bool}, ", &self.in_vap());
            defmt::write!(f, "ico_done: {=bool}, ", &self.ico_done());
            defmt::write!(f, "stat_ac: {=bool}, ", &self.stat_ac());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargerStatus {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargerStatus {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargerStatus {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargerStatus {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargerStatus {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargerStatus {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargerStatus {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotStatus {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ProchotStatus {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ProchotStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 184] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `stat_adapter_removal` field of the register.
        ///
        /// PROCHOT Profile Adapter Removal status. Latched until read by host.
        pub fn stat_adapter_removal(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `stat_battery_removal` field of the register.
        ///
        /// PROCHOT Profile Battery Removal status. Latched until read by host.
        pub fn stat_battery_removal(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `stat_vsys` field of the register.
        ///
        /// PROCHOT Profile VSYS status. Latched until read by host.
        pub fn stat_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `stat_idchg_1` field of the register.
        ///
        /// PROCHOT Profile IDCHG1 status. Latched until read by host.
        pub fn stat_idchg_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `stat_inom` field of the register.
        ///
        /// PROCHOT Profile INOM status. Latched until read by host.
        pub fn stat_inom(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `stat_icrit` field of the register.
        ///
        /// PROCHOT Profile ICRIT status. Latched until read by host.
        pub fn stat_icrit(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `stat_comp` field of the register.
        ///
        /// PROCHOT Profile CMPOUT status. Latched until read by host.
        pub fn stat_comp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `stat_vindpm` field of the register.
        ///
        /// PROCHOT Profile VINDPM status. PROCHOT pin stays low until host writes this status bit to 0 when PP_VINDPM=1.
        pub fn stat_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `stat_exit_vap` field of the register.
        ///
        /// PROCHOT_EXIT_VAP status. When charger exits VAP mode (host disable or faults), PROCHOT pin stays low until host writes this bit to 0.
        pub fn stat_exit_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `stat_vap_fail` field of the register.
        ///
        /// VAP failure status. Reports failure to load VBUS 7 consecutive times in VAP mode. Charger exits VAP and latches off until host writes 0.
        pub fn stat_vap_fail(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `tshut` field of the register.
        ///
        /// Thermal shutdown triggered. Latched until read by host. If TSHUT is still present during read, bit remains set.
        pub fn tshut(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `prochot_clear` field of the register.
        ///
        /// PROCHOT Pulse Clear. Write 0 to clear PROCHOT pulse and drive PROCHOT pin HIGH when EN_PROCHOT_EXT=1.
        pub fn prochot_clear(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `prochot_width` field of the register.
        ///
        /// PROCHOT Pulse Width minimum when EN_PROCHOT_EXT=0.
        pub fn prochot_width(&self) -> super::ProchotPulseWidth {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 14) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_prochot_ext` field of the register.
        ///
        /// PROCHOT Pulse Extension Enable. Keep PROCHOT pin LOW until host writes PROCHOT_CLEAR=0.
        pub fn en_prochot_ext(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Write the `stat_vindpm` field of the register.
        ///
        /// PROCHOT Profile VINDPM status. PROCHOT pin stays low until host writes this status bit to 0 when PP_VINDPM=1.
        pub fn set_stat_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `stat_exit_vap` field of the register.
        ///
        /// PROCHOT_EXIT_VAP status. When charger exits VAP mode (host disable or faults), PROCHOT pin stays low until host writes this bit to 0.
        pub fn set_stat_exit_vap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `stat_vap_fail` field of the register.
        ///
        /// VAP failure status. Reports failure to load VBUS 7 consecutive times in VAP mode. Charger exits VAP and latches off until host writes 0.
        pub fn set_stat_vap_fail(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `prochot_clear` field of the register.
        ///
        /// PROCHOT Pulse Clear. Write 0 to clear PROCHOT pulse and drive PROCHOT pin HIGH when EN_PROCHOT_EXT=1.
        pub fn set_prochot_clear(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `prochot_width` field of the register.
        ///
        /// PROCHOT Pulse Width minimum when EN_PROCHOT_EXT=0.
        pub fn set_prochot_width(&mut self, value: super::ProchotPulseWidth) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 14, &mut self.bits) };
        }
        ///Write the `en_prochot_ext` field of the register.
        ///
        /// PROCHOT Pulse Extension Enable. Keep PROCHOT pin LOW until host writes PROCHOT_CLEAR=0.
        pub fn set_en_prochot_ext(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 14, 15, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ProchotStatus {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotStatus> for [u8; 2] {
        fn from(val: ProchotStatus) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotStatus");
            d.field("stat_adapter_removal", &self.stat_adapter_removal());
            d.field("stat_battery_removal", &self.stat_battery_removal());
            d.field("stat_vsys", &self.stat_vsys());
            d.field("stat_idchg_1", &self.stat_idchg_1());
            d.field("stat_inom", &self.stat_inom());
            d.field("stat_icrit", &self.stat_icrit());
            d.field("stat_comp", &self.stat_comp());
            d.field("stat_vindpm", &self.stat_vindpm());
            d.field("stat_exit_vap", &self.stat_exit_vap());
            d.field("stat_vap_fail", &self.stat_vap_fail());
            d.field("tshut", &self.tshut());
            d.field("prochot_clear", &self.prochot_clear());
            d.field("prochot_width", &self.prochot_width());
            d.field("en_prochot_ext", &self.en_prochot_ext());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotStatus {{ ");
            defmt::write!(f, "stat_adapter_removal: {=bool}, ", &self.stat_adapter_removal());
            defmt::write!(f, "stat_battery_removal: {=bool}, ", &self.stat_battery_removal());
            defmt::write!(f, "stat_vsys: {=bool}, ", &self.stat_vsys());
            defmt::write!(f, "stat_idchg_1: {=bool}, ", &self.stat_idchg_1());
            defmt::write!(f, "stat_inom: {=bool}, ", &self.stat_inom());
            defmt::write!(f, "stat_icrit: {=bool}, ", &self.stat_icrit());
            defmt::write!(f, "stat_comp: {=bool}, ", &self.stat_comp());
            defmt::write!(f, "stat_vindpm: {=bool}, ", &self.stat_vindpm());
            defmt::write!(f, "stat_exit_vap: {=bool}, ", &self.stat_exit_vap());
            defmt::write!(f, "stat_vap_fail: {=bool}, ", &self.stat_vap_fail());
            defmt::write!(f, "tshut: {=bool}, ", &self.tshut());
            defmt::write!(f, "prochot_clear: {=bool}, ", &self.prochot_clear());
            defmt::write!(f, "prochot_width: {}, ", &self.prochot_width());
            defmt::write!(f, "en_prochot_ext: {=bool}, ", &self.en_prochot_ext());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotStatus {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotStatus {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotStatus {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotStatus {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotStatus {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotStatus {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotStatus {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IinDpm {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for IinDpm {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl IinDpm {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 65] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `iin_dpm` field of the register.
        ///
        /// 7-bit actual input current limit in use. With 10 mOhm sense resistor, LSB=50 mA. May differ from IIN_HOST after ICO. 50 mA offset at code 0.
        pub fn iin_dpm(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 15) };
            raw
        }
    }
    impl From<[u8; 2]> for IinDpm {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<IinDpm> for [u8; 2] {
        fn from(val: IinDpm) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for IinDpm {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("IinDpm");
            d.field("iin_dpm", &self.iin_dpm());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for IinDpm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IinDpm {{ ");
            defmt::write!(f, "iin_dpm: {=u8}, ", &self.iin_dpm());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for IinDpm {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for IinDpm {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for IinDpm {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for IinDpm {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for IinDpm {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for IinDpm {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for IinDpm {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcVbusPsys {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcVbusPsys {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcVbusPsys {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `adc_psys` field of the register.
        ///
        /// 8-bit digital output of system power. Full range 3.06 V, LSB 12 mV (ADC_FULLSCALE=1). Full range 2.04 V, LSB 8 mV (ADC_FULLSCALE=0).
        pub fn adc_psys(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Read the `adc_vbus` field of the register.
        ///
        /// 8-bit digital output of input voltage. Full range 0-24480 mV, LSB 96 mV.
        pub fn adc_vbus(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 16) };
            raw
        }
    }
    impl From<[u8; 2]> for AdcVbusPsys {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcVbusPsys> for [u8; 2] {
        fn from(val: AdcVbusPsys) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcVbusPsys {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcVbusPsys");
            d.field("adc_psys", &self.adc_psys());
            d.field("adc_vbus", &self.adc_vbus());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcVbusPsys {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcVbusPsys {{ ");
            defmt::write!(f, "adc_psys: {=u8}, ", &self.adc_psys());
            defmt::write!(f, "adc_vbus: {=u8}, ", &self.adc_vbus());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcVbusPsys {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcVbusPsys {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcVbusPsys {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcVbusPsys {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcVbusPsys {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcVbusPsys {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcVbusPsys {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcIbat {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcIbat {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcIbat {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `adc_idchg` field of the register.
        ///
        /// 7-bit digital output of battery discharge current. With 10 mOhm sense resistor (RSNS_RSR=0), full range 32512 mA, LSB 256 mA. With 5 mOhm (RSNS_RSR=1), full range 65024 mA, LSB 512 mA.
        pub fn adc_idchg(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 7) };
            raw
        }
        ///Read the `adc_ichg` field of the register.
        ///
        /// 7-bit digital output of battery charge current. With 10 mOhm sense resistor (RSNS_RSR=0), full range 8128 mA, LSB 64 mA. With 5 mOhm (RSNS_RSR=1), full range 16256 mA, LSB 128 mA.
        pub fn adc_ichg(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 15) };
            raw
        }
    }
    impl From<[u8; 2]> for AdcIbat {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcIbat> for [u8; 2] {
        fn from(val: AdcIbat) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcIbat {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcIbat");
            d.field("adc_idchg", &self.adc_idchg());
            d.field("adc_ichg", &self.adc_ichg());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcIbat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcIbat {{ ");
            defmt::write!(f, "adc_idchg: {=u8}, ", &self.adc_idchg());
            defmt::write!(f, "adc_ichg: {=u8}, ", &self.adc_ichg());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcIbat {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcIbat {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcIbat {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcIbat {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcIbat {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcIbat {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcIbat {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcIinCmpin {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcIinCmpin {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcIinCmpin {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `adc_cmpin` field of the register.
        ///
        /// 8-bit digital output of CMPIN voltage. Full range 3.06 V, LSB 12 mV (ADC_FULLSCALE=1). Full range 2.04 V, LSB 8 mV (ADC_FULLSCALE=0).
        pub fn adc_cmpin(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Read the `adc_iin` field of the register.
        ///
        /// 8-bit digital output of input current. With 10 mOhm sense resistor (RSNS_RAC=0), full range 12750 mA, LSB 50 mA. With 5 mOhm (RSNS_RAC=1), full range 25500 mA, LSB 100 mA.
        pub fn adc_iin(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 16) };
            raw
        }
    }
    impl From<[u8; 2]> for AdcIinCmpin {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcIinCmpin> for [u8; 2] {
        fn from(val: AdcIinCmpin) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcIinCmpin {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcIinCmpin");
            d.field("adc_cmpin", &self.adc_cmpin());
            d.field("adc_iin", &self.adc_iin());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcIinCmpin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcIinCmpin {{ ");
            defmt::write!(f, "adc_cmpin: {=u8}, ", &self.adc_cmpin());
            defmt::write!(f, "adc_iin: {=u8}, ", &self.adc_iin());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcIinCmpin {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcIinCmpin {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcIinCmpin {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcIinCmpin {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcIinCmpin {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcIinCmpin {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcIinCmpin {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcVsysVbat {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcVsysVbat {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcVsysVbat {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `adc_vbat` field of the register.
        ///
        /// 8-bit digital output of battery voltage. Full range 2.88-19.2 V, LSB 64 mV. Fixed offset 2.88 V.
        pub fn adc_vbat(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Read the `adc_vsys` field of the register.
        ///
        /// 8-bit digital output of system voltage. Full range 2.88-19.2 V, LSB 64 mV. Fixed offset 2.88 V.
        pub fn adc_vsys(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 16) };
            raw
        }
    }
    impl From<[u8; 2]> for AdcVsysVbat {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcVsysVbat> for [u8; 2] {
        fn from(val: AdcVsysVbat) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcVsysVbat {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcVsysVbat");
            d.field("adc_vbat", &self.adc_vbat());
            d.field("adc_vsys", &self.adc_vsys());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcVsysVbat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcVsysVbat {{ ");
            defmt::write!(f, "adc_vbat: {=u8}, ", &self.adc_vbat());
            defmt::write!(f, "adc_vsys: {=u8}, ", &self.adc_vsys());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcVsysVbat {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcVsysVbat {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcVsysVbat {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcVsysVbat {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcVsysVbat {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcVsysVbat {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcVsysVbat {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ManufacturerId {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ManufacturerId {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ManufacturerId {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [64] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `manufacturer_id` field of the register.
        ///
        /// Manufacturer ID. Always returns 0x40.
        pub fn manufacturer_id(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
    }
    impl From<[u8; 1]> for ManufacturerId {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ManufacturerId> for [u8; 1] {
        fn from(val: ManufacturerId) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ManufacturerId {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ManufacturerId");
            d.field("manufacturer_id", &self.manufacturer_id());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ManufacturerId {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ManufacturerId {{ ");
            defmt::write!(f, "manufacturer_id: {=u8}, ", &self.manufacturer_id());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ManufacturerId {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ManufacturerId {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ManufacturerId {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ManufacturerId {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ManufacturerId {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ManufacturerId {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ManufacturerId {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DeviceId {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for DeviceId {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DeviceId {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [224] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `device_id` field of the register.
        ///
        /// Device ID. BQ25723 returns 0xE0 (11010000b).
        pub fn device_id(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
    }
    impl From<[u8; 1]> for DeviceId {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<DeviceId> for [u8; 1] {
        fn from(val: DeviceId) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for DeviceId {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("DeviceId");
            d.field("device_id", &self.device_id());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for DeviceId {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DeviceId {{ ");
            defmt::write!(f, "device_id: {=u8}, ", &self.device_id());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for DeviceId {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for DeviceId {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for DeviceId {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for DeviceId {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for DeviceId {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for DeviceId {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for DeviceId {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption1 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargeOption1 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 51] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `auto_wakeup_en` field of the register.
        ///
        /// Auto Wakeup Enable. If battery is below VSYS_MIN, device automatically enables 128 mA charging current for 30 mins. When battery charges above minimum system voltage, charge terminates and bit resets to LOW.
        pub fn auto_wakeup_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_ship_dchg` field of the register.
        ///
        /// Discharge SRN for Shipping Mode. When set, discharge SRN pin down in 140 ms at 20 mA. Auto-resets to 0 after 140 ms. Used to discharge VBAT pin capacitor voltage for battery gauge device shipping mode.
        pub fn en_ship_dchg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `en_ptm` field of the register.
        ///
        /// PTM (Pass Through Mode) enable. In PTM, buck and boost high-side FETs are both ON, low-side FETs both OFF. Input power passes directly through to system. Automatically resets to zero.
        pub fn en_ptm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `force_conv_off` field of the register.
        ///
        /// Force Converter Off function. When independent comparator triggers (CMPOUT pulled down), converter latches off and CHRG_OK goes LOW. To exit, CMPOUT must be released high and this bit must be cleared.
        pub fn force_conv_off(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `cmp_deg` field of the register.
        ///
        /// Independent comparator deglitch time, applied to falling edge of CMPOUT (HIGH to LOW).
        pub fn cmp_deg(&self) -> super::ComparatorDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `cmp_pol` field of the register.
        ///
        /// Independent Comparator output Polarity. 0=CMPIN above threshold makes CMPOUT LOW (internal hysteresis). 1=CMPIN below threshold makes CMPOUT LOW (external hysteresis).
        pub fn cmp_pol(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `cmp_ref` field of the register.
        ///
        /// Independent Comparator internal Reference. 0=2.3 V, 1=1.2 V.
        pub fn cmp_ref(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `en_fast_5_mohm` field of the register.
        ///
        /// Enable fast compensation to increase bandwidth under 5 mOhm RAC (RSNS_RAC=1) for input current up to 6.4 A. When 0, IIN_HOST DAC extends to 10 A but ICHG loop is slower. When 1, IIN_HOST DAC clamped at 6.4 A with faster ICHG loop.
        pub fn en_fast_5_mohm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `psys_ratio` field of the register.
        ///
        /// PSYS Gain. Ratio of PSYS output current vs total system power. 0=0.25 uA/W, 1=1 uA/W.
        pub fn psys_ratio(&self) -> super::PsysGain {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `rsns_rsr` field of the register.
        ///
        /// Charge sense resistor RSR. 0=10 mOhm, 1=5 mOhm.
        pub fn rsns_rsr(&self) -> super::ChargeSenseResistor {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `rsns_rac` field of the register.
        ///
        /// Input sense resistor RAC. 0=10 mOhm, 1=5 mOhm.
        pub fn rsns_rac(&self) -> super::InputSenseResistor {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `psys_config` field of the register.
        ///
        /// PSYS Enable and Definition. In low power mode (EN_LWPWR=1), PSYS sensing and buffer are always disabled regardless of this value. 00=PBUS+PBAT, 01=PBUS, 10=Reserved, 11=Off (minimize Iq).
        pub fn psys_config(&self) -> super::PsysConfig {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 14) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_prochot_lpwr` field of the register.
        ///
        /// Enable PROCHOT during battery only low power mode. Uses independent comparator to monitor system voltage with low power consumption. Do not enable with adapter present.
        pub fn en_prochot_lpwr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `en_ibat` field of the register.
        ///
        /// IBAT Enable. Enable the IBAT output buffer. In low power mode (EN_LWPWR=1), IBAT buffer is always disabled regardless of this value.
        pub fn en_ibat(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Write the `auto_wakeup_en` field of the register.
        ///
        /// Auto Wakeup Enable. If battery is below VSYS_MIN, device automatically enables 128 mA charging current for 30 mins. When battery charges above minimum system voltage, charge terminates and bit resets to LOW.
        pub fn set_auto_wakeup_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_ship_dchg` field of the register.
        ///
        /// Discharge SRN for Shipping Mode. When set, discharge SRN pin down in 140 ms at 20 mA. Auto-resets to 0 after 140 ms. Used to discharge VBAT pin capacitor voltage for battery gauge device shipping mode.
        pub fn set_en_ship_dchg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_ptm` field of the register.
        ///
        /// PTM (Pass Through Mode) enable. In PTM, buck and boost high-side FETs are both ON, low-side FETs both OFF. Input power passes directly through to system. Automatically resets to zero.
        pub fn set_en_ptm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `force_conv_off` field of the register.
        ///
        /// Force Converter Off function. When independent comparator triggers (CMPOUT pulled down), converter latches off and CHRG_OK goes LOW. To exit, CMPOUT must be released high and this bit must be cleared.
        pub fn set_force_conv_off(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `cmp_deg` field of the register.
        ///
        /// Independent comparator deglitch time, applied to falling edge of CMPOUT (HIGH to LOW).
        pub fn set_cmp_deg(&mut self, value: super::ComparatorDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 6, &mut self.bits) };
        }
        ///Write the `cmp_pol` field of the register.
        ///
        /// Independent Comparator output Polarity. 0=CMPIN above threshold makes CMPOUT LOW (internal hysteresis). 1=CMPIN below threshold makes CMPOUT LOW (external hysteresis).
        pub fn set_cmp_pol(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `cmp_ref` field of the register.
        ///
        /// Independent Comparator internal Reference. 0=2.3 V, 1=1.2 V.
        pub fn set_cmp_ref(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `en_fast_5_mohm` field of the register.
        ///
        /// Enable fast compensation to increase bandwidth under 5 mOhm RAC (RSNS_RAC=1) for input current up to 6.4 A. When 0, IIN_HOST DAC extends to 10 A but ICHG loop is slower. When 1, IIN_HOST DAC clamped at 6.4 A with faster ICHG loop.
        pub fn set_en_fast_5_mohm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `psys_ratio` field of the register.
        ///
        /// PSYS Gain. Ratio of PSYS output current vs total system power. 0=0.25 uA/W, 1=1 uA/W.
        pub fn set_psys_ratio(&mut self, value: super::PsysGain) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `rsns_rsr` field of the register.
        ///
        /// Charge sense resistor RSR. 0=10 mOhm, 1=5 mOhm.
        pub fn set_rsns_rsr(&mut self, value: super::ChargeSenseResistor) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `rsns_rac` field of the register.
        ///
        /// Input sense resistor RAC. 0=10 mOhm, 1=5 mOhm.
        pub fn set_rsns_rac(&mut self, value: super::InputSenseResistor) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `psys_config` field of the register.
        ///
        /// PSYS Enable and Definition. In low power mode (EN_LWPWR=1), PSYS sensing and buffer are always disabled regardless of this value. 00=PBUS+PBAT, 01=PBUS, 10=Reserved, 11=Off (minimize Iq).
        pub fn set_psys_config(&mut self, value: super::PsysConfig) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 14, &mut self.bits) };
        }
        ///Write the `en_prochot_lpwr` field of the register.
        ///
        /// Enable PROCHOT during battery only low power mode. Uses independent comparator to monitor system voltage with low power consumption. Do not enable with adapter present.
        pub fn set_en_prochot_lpwr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 14, 15, &mut self.bits) };
        }
        ///Write the `en_ibat` field of the register.
        ///
        /// IBAT Enable. Enable the IBAT output buffer. In low power mode (EN_LWPWR=1), IBAT buffer is always disabled regardless of this value.
        pub fn set_en_ibat(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ChargeOption1 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption1> for [u8; 2] {
        fn from(val: ChargeOption1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption1");
            d.field("auto_wakeup_en", &self.auto_wakeup_en());
            d.field("en_ship_dchg", &self.en_ship_dchg());
            d.field("en_ptm", &self.en_ptm());
            d.field("force_conv_off", &self.force_conv_off());
            d.field("cmp_deg", &self.cmp_deg());
            d.field("cmp_pol", &self.cmp_pol());
            d.field("cmp_ref", &self.cmp_ref());
            d.field("en_fast_5_mohm", &self.en_fast_5_mohm());
            d.field("psys_ratio", &self.psys_ratio());
            d.field("rsns_rsr", &self.rsns_rsr());
            d.field("rsns_rac", &self.rsns_rac());
            d.field("psys_config", &self.psys_config());
            d.field("en_prochot_lpwr", &self.en_prochot_lpwr());
            d.field("en_ibat", &self.en_ibat());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption1 {{ ");
            defmt::write!(f, "auto_wakeup_en: {=bool}, ", &self.auto_wakeup_en());
            defmt::write!(f, "en_ship_dchg: {=bool}, ", &self.en_ship_dchg());
            defmt::write!(f, "en_ptm: {=bool}, ", &self.en_ptm());
            defmt::write!(f, "force_conv_off: {=bool}, ", &self.force_conv_off());
            defmt::write!(f, "cmp_deg: {}, ", &self.cmp_deg());
            defmt::write!(f, "cmp_pol: {=bool}, ", &self.cmp_pol());
            defmt::write!(f, "cmp_ref: {=bool}, ", &self.cmp_ref());
            defmt::write!(f, "en_fast_5_mohm: {=bool}, ", &self.en_fast_5_mohm());
            defmt::write!(f, "psys_ratio: {}, ", &self.psys_ratio());
            defmt::write!(f, "rsns_rsr: {}, ", &self.rsns_rsr());
            defmt::write!(f, "rsns_rac: {}, ", &self.rsns_rac());
            defmt::write!(f, "psys_config: {}, ", &self.psys_config());
            defmt::write!(f, "en_prochot_lpwr: {=bool}, ", &self.en_prochot_lpwr());
            defmt::write!(f, "en_ibat: {=bool}, ", &self.en_ibat());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption2 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargeOption2 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [183, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `batoc_vth` field of the register.
        ///
        /// Battery discharge overcurrent threshold as percentage of PROCHOT IDCHG_TH2. Note for 1S application, threshold could derate at low SRN/SRP common voltage.
        pub fn batoc_vth(&self) -> super::BatocThreshold {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_batoc` field of the register.
        ///
        /// Battery discharge overcurrent (BATOC) protection enable. Upon BATOC, converter is disabled. Non-latch fault, resumes after 250 ms relax time.
        pub fn en_batoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `acoc_vth` field of the register.
        ///
        /// ACOC Limit. Set ACOC threshold as percentage of ILIM2. 0=133% of ILIM2, 1=200% of ILIM2.
        pub fn acoc_vth(&self) -> super::AcocThreshold {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_acoc` field of the register.
        ///
        /// Input overcurrent (ACOC) protection enable. Non-latch fault with 250 us rising edge deglitch. After input current falls below threshold, 250 ms falling edge deglitch before converter restarts.
        pub fn en_acoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `acx_ocp` field of the register.
        ///
        /// Fixed input current OCP threshold by sensing ACP-ACN. 0=280 mV (RSNS_RAC=0) / 200 mV (RSNS_RAC=1). 1=150 mV (RSNS_RAC=0) / 100 mV (RSNS_RAC=1).
        pub fn acx_ocp(&self) -> super::AcxOcpThreshold {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `q_2_ocp` field of the register.
        ///
        /// Q2 OCP threshold by sensing Q2 VDS.
        pub fn q_2_ocp(&self) -> super::Q2OcpThreshold {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_ichg_idchg` field of the register.
        ///
        /// IBAT pin monitor selection. 0=IBAT as discharge current, 1=IBAT as charge current.
        pub fn en_ichg_idchg(&self) -> super::IBatPinSelect {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_extilim` field of the register.
        ///
        /// Enable ILIM_HIZ pin to set input current limit. When enabled, input current limit is the lower of ILIM_HIZ pin and IIN_DPM register.
        pub fn en_extilim(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `pkpwr_tmax` field of the register.
        ///
        /// Peak power mode overload and relax cycle time.
        pub fn pkpwr_tmax(&self) -> super::PkpwrTmax {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 10) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pkpwr_relax_stat` field of the register.
        ///
        /// Indicator that the device is in relaxation cycle. Write 0 to exit.
        pub fn pkpwr_relax_stat(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `stat_pkpwr_ovld` field of the register.
        ///
        /// Indicator that the device is in overloading cycle. Write 0 to exit.
        pub fn stat_pkpwr_ovld(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `en_pkpwr_vsys` field of the register.
        ///
        /// Enable Peak Power Mode triggered by system voltage under-shoot. Threshold is VSYS_MIN register setting. Upon adapter removal, reset to 0.
        pub fn en_pkpwr_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `en_pkpwr_iin_dpm` field of the register.
        ///
        /// Enable Peak Power Mode triggered by input current overshoot. Threshold is IIN_DPM register. Upon adapter removal, reset to 0.
        pub fn en_pkpwr_iin_dpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `pkpwr_tovld_deg` field of the register.
        ///
        /// Input Overload time (ILIM2 effective duration) in Peak Power Mode.
        pub fn pkpwr_tovld_deg(&self) -> super::PkpwrTovldDeg {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 16) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `batoc_vth` field of the register.
        ///
        /// Battery discharge overcurrent threshold as percentage of PROCHOT IDCHG_TH2. Note for 1S application, threshold could derate at low SRN/SRP common voltage.
        pub fn set_batoc_vth(&mut self, value: super::BatocThreshold) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_batoc` field of the register.
        ///
        /// Battery discharge overcurrent (BATOC) protection enable. Upon BATOC, converter is disabled. Non-latch fault, resumes after 250 ms relax time.
        pub fn set_en_batoc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `acoc_vth` field of the register.
        ///
        /// ACOC Limit. Set ACOC threshold as percentage of ILIM2. 0=133% of ILIM2, 1=200% of ILIM2.
        pub fn set_acoc_vth(&mut self, value: super::AcocThreshold) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `en_acoc` field of the register.
        ///
        /// Input overcurrent (ACOC) protection enable. Non-latch fault with 250 us rising edge deglitch. After input current falls below threshold, 250 ms falling edge deglitch before converter restarts.
        pub fn set_en_acoc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `acx_ocp` field of the register.
        ///
        /// Fixed input current OCP threshold by sensing ACP-ACN. 0=280 mV (RSNS_RAC=0) / 200 mV (RSNS_RAC=1). 1=150 mV (RSNS_RAC=0) / 100 mV (RSNS_RAC=1).
        pub fn set_acx_ocp(&mut self, value: super::AcxOcpThreshold) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `q_2_ocp` field of the register.
        ///
        /// Q2 OCP threshold by sensing Q2 VDS.
        pub fn set_q_2_ocp(&mut self, value: super::Q2OcpThreshold) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `en_ichg_idchg` field of the register.
        ///
        /// IBAT pin monitor selection. 0=IBAT as discharge current, 1=IBAT as charge current.
        pub fn set_en_ichg_idchg(&mut self, value: super::IBatPinSelect) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_extilim` field of the register.
        ///
        /// Enable ILIM_HIZ pin to set input current limit. When enabled, input current limit is the lower of ILIM_HIZ pin and IIN_DPM register.
        pub fn set_en_extilim(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `pkpwr_tmax` field of the register.
        ///
        /// Peak power mode overload and relax cycle time.
        pub fn set_pkpwr_tmax(&mut self, value: super::PkpwrTmax) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 10, &mut self.bits) };
        }
        ///Write the `pkpwr_relax_stat` field of the register.
        ///
        /// Indicator that the device is in relaxation cycle. Write 0 to exit.
        pub fn set_pkpwr_relax_stat(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `stat_pkpwr_ovld` field of the register.
        ///
        /// Indicator that the device is in overloading cycle. Write 0 to exit.
        pub fn set_stat_pkpwr_ovld(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `en_pkpwr_vsys` field of the register.
        ///
        /// Enable Peak Power Mode triggered by system voltage under-shoot. Threshold is VSYS_MIN register setting. Upon adapter removal, reset to 0.
        pub fn set_en_pkpwr_vsys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 13, &mut self.bits) };
        }
        ///Write the `en_pkpwr_iin_dpm` field of the register.
        ///
        /// Enable Peak Power Mode triggered by input current overshoot. Threshold is IIN_DPM register. Upon adapter removal, reset to 0.
        pub fn set_en_pkpwr_iin_dpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 13, 14, &mut self.bits) };
        }
        ///Write the `pkpwr_tovld_deg` field of the register.
        ///
        /// Input Overload time (ILIM2 effective duration) in Peak Power Mode.
        pub fn set_pkpwr_tovld_deg(&mut self, value: super::PkpwrTovldDeg) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 14, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ChargeOption2 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption2> for [u8; 2] {
        fn from(val: ChargeOption2) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption2 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption2");
            d.field("batoc_vth", &self.batoc_vth());
            d.field("en_batoc", &self.en_batoc());
            d.field("acoc_vth", &self.acoc_vth());
            d.field("en_acoc", &self.en_acoc());
            d.field("acx_ocp", &self.acx_ocp());
            d.field("q_2_ocp", &self.q_2_ocp());
            d.field("en_ichg_idchg", &self.en_ichg_idchg());
            d.field("en_extilim", &self.en_extilim());
            d.field("pkpwr_tmax", &self.pkpwr_tmax());
            d.field("pkpwr_relax_stat", &self.pkpwr_relax_stat());
            d.field("stat_pkpwr_ovld", &self.stat_pkpwr_ovld());
            d.field("en_pkpwr_vsys", &self.en_pkpwr_vsys());
            d.field("en_pkpwr_iin_dpm", &self.en_pkpwr_iin_dpm());
            d.field("pkpwr_tovld_deg", &self.pkpwr_tovld_deg());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption2 {{ ");
            defmt::write!(f, "batoc_vth: {}, ", &self.batoc_vth());
            defmt::write!(f, "en_batoc: {=bool}, ", &self.en_batoc());
            defmt::write!(f, "acoc_vth: {}, ", &self.acoc_vth());
            defmt::write!(f, "en_acoc: {=bool}, ", &self.en_acoc());
            defmt::write!(f, "acx_ocp: {}, ", &self.acx_ocp());
            defmt::write!(f, "q_2_ocp: {}, ", &self.q_2_ocp());
            defmt::write!(f, "en_ichg_idchg: {}, ", &self.en_ichg_idchg());
            defmt::write!(f, "en_extilim: {=bool}, ", &self.en_extilim());
            defmt::write!(f, "pkpwr_tmax: {}, ", &self.pkpwr_tmax());
            defmt::write!(f, "pkpwr_relax_stat: {=bool}, ", &self.pkpwr_relax_stat());
            defmt::write!(f, "stat_pkpwr_ovld: {=bool}, ", &self.stat_pkpwr_ovld());
            defmt::write!(f, "en_pkpwr_vsys: {=bool}, ", &self.en_pkpwr_vsys());
            defmt::write!(f, "en_pkpwr_iin_dpm: {=bool}, ", &self.en_pkpwr_iin_dpm());
            defmt::write!(f, "pkpwr_tovld_deg: {}, ", &self.pkpwr_tovld_deg());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption2 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption2 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption2 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption2 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption2 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption2 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption2 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption3 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargeOption3 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [52, 4] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `psys_otg_idchg` field of the register.
        ///
        /// PSYS function during OTG mode. 0=battery discharge power minus OTG output power. 1=battery discharge power only.
        pub fn psys_otg_idchg(&self) -> super::PsysOtgIdchg {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `batfetoff_hiz` field of the register.
        ///
        /// Control BATFET during HIZ mode. 0=BATFET on, 1=BATFET off.
        pub fn batfetoff_hiz(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `cmp_en` field of the register.
        ///
        /// Enable Independent Comparator with effective low.
        pub fn cmp_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `il_avg` field of the register.
        ///
        /// Converter inductor average current clamp. Choose the smallest option higher than maximum possible converter average inductor current.
        pub fn il_avg(&self) -> super::InductorAvgCurrentClamp {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `otg_vap_mode` field of the register.
        ///
        /// External OTG/VAP/FRS pin control selection. 0=pin controls VAP mode. 1=pin controls OTG mode. Do not change after OTG/VAP/FRS pin is pulled high.
        pub fn otg_vap_mode(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `en_vbus_vap` field of the register.
        ///
        /// Enable the VBUS VAP for VAP operation mode 2 and 3.
        pub fn en_vbus_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `batfet_enz` field of the register.
        ///
        /// Turn off BATFET under battery only mode. Not allowed when not in battery only mode. Forced to 0 in battery only OTG mode.
        pub fn batfet_enz(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `en_otg_bigcap` field of the register.
        ///
        /// Enable OTG compensation for VBUS effective capacitance larger than 33 uF. 0=for capacitance smaller than 33 uF, 1=for larger than 33 uF.
        pub fn en_otg_bigcap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `en_vsys_min_soft_sr` field of the register.
        ///
        /// Enable VSYS_MIN soft slew rate transition (1 LSB/8 us = 12.5 mV/us).
        pub fn en_vsys_min_soft_sr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `en_port_ctrl` field of the register.
        ///
        /// Enable BATFET control pin by activating BATDRV pin. 0=Disable BATFET control (HIZ BATDRV pin). 1=Enable BATFET control.
        pub fn en_port_ctrl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `en_ico_mode` field of the register.
        ///
        /// Enable ICO (Input Current Optimizer) Algorithm. Automatically detects optimized input current limit. Requires adapter input current limit of at least 500 mA.
        pub fn en_ico_mode(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `en_otg` field of the register.
        ///
        /// OTG Mode Enable. Enable device in OTG mode when OTG/VAP/FRS pin is HIGH. Set OTG voltage/current registers before enabling.
        pub fn en_otg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `reset_vindpm` field of the register.
        ///
        /// Reset VINDPM Threshold. Converter is disabled to measure VINDPM threshold. Auto-resets to 0 when measurement is done. Not recommended when battery voltage is lower than VSYS_MIN.
        pub fn reset_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `reset_reg` field of the register.
        ///
        /// Reset all registers to POR default except VINDPM register. Auto-resets to 0 after reset.
        pub fn reset_reg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `en_hiz` field of the register.
        ///
        /// Device HIZ Mode Enable. Device draws minimal quiescent current with VBUS above UVLO. REGN LDO stays on, system powers from battery. ADC circuits disabled.
        pub fn en_hiz(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Write the `psys_otg_idchg` field of the register.
        ///
        /// PSYS function during OTG mode. 0=battery discharge power minus OTG output power. 1=battery discharge power only.
        pub fn set_psys_otg_idchg(&mut self, value: super::PsysOtgIdchg) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `batfetoff_hiz` field of the register.
        ///
        /// Control BATFET during HIZ mode. 0=BATFET on, 1=BATFET off.
        pub fn set_batfetoff_hiz(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `cmp_en` field of the register.
        ///
        /// Enable Independent Comparator with effective low.
        pub fn set_cmp_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `il_avg` field of the register.
        ///
        /// Converter inductor average current clamp. Choose the smallest option higher than maximum possible converter average inductor current.
        pub fn set_il_avg(&mut self, value: super::InductorAvgCurrentClamp) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 5, &mut self.bits) };
        }
        ///Write the `otg_vap_mode` field of the register.
        ///
        /// External OTG/VAP/FRS pin control selection. 0=pin controls VAP mode. 1=pin controls OTG mode. Do not change after OTG/VAP/FRS pin is pulled high.
        pub fn set_otg_vap_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `en_vbus_vap` field of the register.
        ///
        /// Enable the VBUS VAP for VAP operation mode 2 and 3.
        pub fn set_en_vbus_vap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `batfet_enz` field of the register.
        ///
        /// Turn off BATFET under battery only mode. Not allowed when not in battery only mode. Forced to 0 in battery only OTG mode.
        pub fn set_batfet_enz(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `en_otg_bigcap` field of the register.
        ///
        /// Enable OTG compensation for VBUS effective capacitance larger than 33 uF. 0=for capacitance smaller than 33 uF, 1=for larger than 33 uF.
        pub fn set_en_otg_bigcap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `en_vsys_min_soft_sr` field of the register.
        ///
        /// Enable VSYS_MIN soft slew rate transition (1 LSB/8 us = 12.5 mV/us).
        pub fn set_en_vsys_min_soft_sr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `en_port_ctrl` field of the register.
        ///
        /// Enable BATFET control pin by activating BATDRV pin. 0=Disable BATFET control (HIZ BATDRV pin). 1=Enable BATFET control.
        pub fn set_en_port_ctrl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `en_ico_mode` field of the register.
        ///
        /// Enable ICO (Input Current Optimizer) Algorithm. Automatically detects optimized input current limit. Requires adapter input current limit of at least 500 mA.
        pub fn set_en_ico_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `en_otg` field of the register.
        ///
        /// OTG Mode Enable. Enable device in OTG mode when OTG/VAP/FRS pin is HIGH. Set OTG voltage/current registers before enabling.
        pub fn set_en_otg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 13, &mut self.bits) };
        }
        ///Write the `reset_vindpm` field of the register.
        ///
        /// Reset VINDPM Threshold. Converter is disabled to measure VINDPM threshold. Auto-resets to 0 when measurement is done. Not recommended when battery voltage is lower than VSYS_MIN.
        pub fn set_reset_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 13, 14, &mut self.bits) };
        }
        ///Write the `reset_reg` field of the register.
        ///
        /// Reset all registers to POR default except VINDPM register. Auto-resets to 0 after reset.
        pub fn set_reset_reg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 14, 15, &mut self.bits) };
        }
        ///Write the `en_hiz` field of the register.
        ///
        /// Device HIZ Mode Enable. Device draws minimal quiescent current with VBUS above UVLO. REGN LDO stays on, system powers from battery. ADC circuits disabled.
        pub fn set_en_hiz(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ChargeOption3 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption3> for [u8; 2] {
        fn from(val: ChargeOption3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption3");
            d.field("psys_otg_idchg", &self.psys_otg_idchg());
            d.field("batfetoff_hiz", &self.batfetoff_hiz());
            d.field("cmp_en", &self.cmp_en());
            d.field("il_avg", &self.il_avg());
            d.field("otg_vap_mode", &self.otg_vap_mode());
            d.field("en_vbus_vap", &self.en_vbus_vap());
            d.field("batfet_enz", &self.batfet_enz());
            d.field("en_otg_bigcap", &self.en_otg_bigcap());
            d.field("en_vsys_min_soft_sr", &self.en_vsys_min_soft_sr());
            d.field("en_port_ctrl", &self.en_port_ctrl());
            d.field("en_ico_mode", &self.en_ico_mode());
            d.field("en_otg", &self.en_otg());
            d.field("reset_vindpm", &self.reset_vindpm());
            d.field("reset_reg", &self.reset_reg());
            d.field("en_hiz", &self.en_hiz());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption3 {{ ");
            defmt::write!(f, "psys_otg_idchg: {}, ", &self.psys_otg_idchg());
            defmt::write!(f, "batfetoff_hiz: {=bool}, ", &self.batfetoff_hiz());
            defmt::write!(f, "cmp_en: {=bool}, ", &self.cmp_en());
            defmt::write!(f, "il_avg: {}, ", &self.il_avg());
            defmt::write!(f, "otg_vap_mode: {=bool}, ", &self.otg_vap_mode());
            defmt::write!(f, "en_vbus_vap: {=bool}, ", &self.en_vbus_vap());
            defmt::write!(f, "batfet_enz: {=bool}, ", &self.batfet_enz());
            defmt::write!(f, "en_otg_bigcap: {=bool}, ", &self.en_otg_bigcap());
            defmt::write!(f, "en_vsys_min_soft_sr: {=bool}, ", &self.en_vsys_min_soft_sr());
            defmt::write!(f, "en_port_ctrl: {=bool}, ", &self.en_port_ctrl());
            defmt::write!(f, "en_ico_mode: {=bool}, ", &self.en_ico_mode());
            defmt::write!(f, "en_otg: {=bool}, ", &self.en_otg());
            defmt::write!(f, "reset_vindpm: {=bool}, ", &self.reset_vindpm());
            defmt::write!(f, "reset_reg: {=bool}, ", &self.reset_reg());
            defmt::write!(f, "en_hiz: {=bool}, ", &self.en_hiz());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption3 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption3 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption3 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption3 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption3 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption3 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption3 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotOption0 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ProchotOption0 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ProchotOption0 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [129, 74] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `lower_prochot_vindpm` field of the register.
        ///
        /// Enable lower threshold of PROCHOT_VINDPM comparator. 0=threshold follows VINDPM register setting. 1=threshold determined by PROCHOT_VINDPM_80_90 bit.
        pub fn lower_prochot_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `inom_deg` field of the register.
        ///
        /// INOM Deglitch Time. INOM is always 10% above IIN_DPM register setting.
        pub fn inom_deg(&self) -> super::InomDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vsys_th_1` field of the register.
        ///
        /// VSYS Threshold 1 to trigger discharging VBUS in VAP mode. 6-bit, fixed 5 us deglitch. Triggers when VSYS pin voltage is below threshold. Fixed DC offset 3.2 V, LSB 100 mV. 2S-4S range 3.2-9.5 V (default 6.4 V). 1S range 3.2-3.9 V (default 3.4 V).
        pub fn vsys_th_1(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 8) };
            raw
        }
        ///Read the `prochot_vindpm_80_90` field of the register.
        ///
        /// Lower threshold of PROCHOT_VINDPM comparator when LOWER_PROCHOT_VINDPM=1. 0=83% of VINDPM, 1=91% of VINDPM.
        pub fn prochot_vindpm_80_90(&self) -> super::ProchotVindpmThreshold {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `icrit_deg` field of the register.
        ///
        /// ICRIT deglitch time to trigger PROCHOT. ICRIT threshold is 110% of ILIM2.
        pub fn icrit_deg(&self) -> super::IcritDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 11) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `ilim_2_vth` field of the register.
        ///
        /// ILIM2 Threshold. 5-bit, percentage of IIN_DPM. 00001-11001 = 110%-230% (5% step). 11010-11110 = 250%-450% (50% step). 11111 = Out of Range (ignored). Default 150% (01001b).
        pub fn ilim_2_vth(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 16) };
            raw
        }
        ///Write the `lower_prochot_vindpm` field of the register.
        ///
        /// Enable lower threshold of PROCHOT_VINDPM comparator. 0=threshold follows VINDPM register setting. 1=threshold determined by PROCHOT_VINDPM_80_90 bit.
        pub fn set_lower_prochot_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `inom_deg` field of the register.
        ///
        /// INOM Deglitch Time. INOM is always 10% above IIN_DPM register setting.
        pub fn set_inom_deg(&mut self, value: super::InomDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `vsys_th_1` field of the register.
        ///
        /// VSYS Threshold 1 to trigger discharging VBUS in VAP mode. 6-bit, fixed 5 us deglitch. Triggers when VSYS pin voltage is below threshold. Fixed DC offset 3.2 V, LSB 100 mV. 2S-4S range 3.2-9.5 V (default 6.4 V). 1S range 3.2-3.9 V (default 3.4 V).
        pub fn set_vsys_th_1(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 8, &mut self.bits) };
        }
        ///Write the `prochot_vindpm_80_90` field of the register.
        ///
        /// Lower threshold of PROCHOT_VINDPM comparator when LOWER_PROCHOT_VINDPM=1. 0=83% of VINDPM, 1=91% of VINDPM.
        pub fn set_prochot_vindpm_80_90(&mut self, value: super::ProchotVindpmThreshold) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `icrit_deg` field of the register.
        ///
        /// ICRIT deglitch time to trigger PROCHOT. ICRIT threshold is 110% of ILIM2.
        pub fn set_icrit_deg(&mut self, value: super::IcritDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 11, &mut self.bits) };
        }
        ///Write the `ilim_2_vth` field of the register.
        ///
        /// ILIM2 Threshold. 5-bit, percentage of IIN_DPM. 00001-11001 = 110%-230% (5% step). 11010-11110 = 250%-450% (50% step). 11111 = Out of Range (ignored). Default 150% (01001b).
        pub fn set_ilim_2_vth(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ProchotOption0 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotOption0> for [u8; 2] {
        fn from(val: ProchotOption0) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotOption0 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotOption0");
            d.field("lower_prochot_vindpm", &self.lower_prochot_vindpm());
            d.field("inom_deg", &self.inom_deg());
            d.field("vsys_th_1", &self.vsys_th_1());
            d.field("prochot_vindpm_80_90", &self.prochot_vindpm_80_90());
            d.field("icrit_deg", &self.icrit_deg());
            d.field("ilim_2_vth", &self.ilim_2_vth());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotOption0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotOption0 {{ ");
            defmt::write!(f, "lower_prochot_vindpm: {=bool}, ", &self.lower_prochot_vindpm());
            defmt::write!(f, "inom_deg: {}, ", &self.inom_deg());
            defmt::write!(f, "vsys_th_1: {=u8}, ", &self.vsys_th_1());
            defmt::write!(f, "prochot_vindpm_80_90: {}, ", &self.prochot_vindpm_80_90());
            defmt::write!(f, "icrit_deg: {}, ", &self.icrit_deg());
            defmt::write!(f, "ilim_2_vth: {=u8}, ", &self.ilim_2_vth());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotOption0 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotOption0 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotOption0 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotOption0 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotOption0 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotOption0 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotOption0 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotOption1 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ProchotOption1 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ProchotOption1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [160, 65] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `pp_acok` field of the register.
        ///
        /// Adapter removal PROCHOT Profile enable. EN_LWPWR=0 required to assert PROCHOT pulse after adapter removal.
        pub fn pp_acok(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `pp_batpres` field of the register.
        ///
        /// Battery removal PROCHOT Profile enable (one-shot falling edge triggered). Immediately sends PROCHOT pulse after battery removal.
        pub fn pp_batpres(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `pp_vsys` field of the register.
        ///
        /// VSYS PROCHOT Profile enable.
        pub fn pp_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `pp_idchg_1` field of the register.
        ///
        /// IDCHG1 PROCHOT Profile enable.
        pub fn pp_idchg_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `pp_inom` field of the register.
        ///
        /// INOM PROCHOT Profile enable.
        pub fn pp_inom(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `pp_icrit` field of the register.
        ///
        /// ICRIT PROCHOT Profile enable.
        pub fn pp_icrit(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `pp_comp` field of the register.
        ///
        /// Independent comparator PROCHOT Profile enable. In low power mode (battery only), use EN_PROCHOT_LPWR instead.
        pub fn pp_comp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `pp_vindpm` field of the register.
        ///
        /// VINDPM PROCHOT Profile enable.
        pub fn pp_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `idchg_deg_1` field of the register.
        ///
        /// IDCHG level 1 Deglitch Time.
        pub fn idchg_deg_1(&self) -> super::IdchgDeg1Time {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 10) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `idchg_th_1` field of the register.
        ///
        /// IDCHG level 1 Threshold. 6-bit, range 0-32256 mA, LSB 512 mA. If 000000b, PROCHOT is always triggered. Default 8192 mA (010000b).
        pub fn idchg_th_1(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 16) };
            raw
        }
        ///Write the `pp_acok` field of the register.
        ///
        /// Adapter removal PROCHOT Profile enable. EN_LWPWR=0 required to assert PROCHOT pulse after adapter removal.
        pub fn set_pp_acok(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `pp_batpres` field of the register.
        ///
        /// Battery removal PROCHOT Profile enable (one-shot falling edge triggered). Immediately sends PROCHOT pulse after battery removal.
        pub fn set_pp_batpres(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `pp_vsys` field of the register.
        ///
        /// VSYS PROCHOT Profile enable.
        pub fn set_pp_vsys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `pp_idchg_1` field of the register.
        ///
        /// IDCHG1 PROCHOT Profile enable.
        pub fn set_pp_idchg_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `pp_inom` field of the register.
        ///
        /// INOM PROCHOT Profile enable.
        pub fn set_pp_inom(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `pp_icrit` field of the register.
        ///
        /// ICRIT PROCHOT Profile enable.
        pub fn set_pp_icrit(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `pp_comp` field of the register.
        ///
        /// Independent comparator PROCHOT Profile enable. In low power mode (battery only), use EN_PROCHOT_LPWR instead.
        pub fn set_pp_comp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `pp_vindpm` field of the register.
        ///
        /// VINDPM PROCHOT Profile enable.
        pub fn set_pp_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `idchg_deg_1` field of the register.
        ///
        /// IDCHG level 1 Deglitch Time.
        pub fn set_idchg_deg_1(&mut self, value: super::IdchgDeg1Time) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 10, &mut self.bits) };
        }
        ///Write the `idchg_th_1` field of the register.
        ///
        /// IDCHG level 1 Threshold. 6-bit, range 0-32256 mA, LSB 512 mA. If 000000b, PROCHOT is always triggered. Default 8192 mA (010000b).
        pub fn set_idchg_th_1(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ProchotOption1 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotOption1> for [u8; 2] {
        fn from(val: ProchotOption1) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotOption1 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotOption1");
            d.field("pp_acok", &self.pp_acok());
            d.field("pp_batpres", &self.pp_batpres());
            d.field("pp_vsys", &self.pp_vsys());
            d.field("pp_idchg_1", &self.pp_idchg_1());
            d.field("pp_inom", &self.pp_inom());
            d.field("pp_icrit", &self.pp_icrit());
            d.field("pp_comp", &self.pp_comp());
            d.field("pp_vindpm", &self.pp_vindpm());
            d.field("idchg_deg_1", &self.idchg_deg_1());
            d.field("idchg_th_1", &self.idchg_th_1());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotOption1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotOption1 {{ ");
            defmt::write!(f, "pp_acok: {=bool}, ", &self.pp_acok());
            defmt::write!(f, "pp_batpres: {=bool}, ", &self.pp_batpres());
            defmt::write!(f, "pp_vsys: {=bool}, ", &self.pp_vsys());
            defmt::write!(f, "pp_idchg_1: {=bool}, ", &self.pp_idchg_1());
            defmt::write!(f, "pp_inom: {=bool}, ", &self.pp_inom());
            defmt::write!(f, "pp_icrit: {=bool}, ", &self.pp_icrit());
            defmt::write!(f, "pp_comp: {=bool}, ", &self.pp_comp());
            defmt::write!(f, "pp_vindpm: {=bool}, ", &self.pp_vindpm());
            defmt::write!(f, "idchg_deg_1: {}, ", &self.idchg_deg_1());
            defmt::write!(f, "idchg_th_1: {=u8}, ", &self.idchg_th_1());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotOption1 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotOption1 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotOption1 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotOption1 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotOption1 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotOption1 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotOption1 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcOption {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AdcOption {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AdcOption {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 32] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `en_adc_vbat` field of the register.
        ///
        /// Enable battery voltage (SRN pin) ADC channel.
        pub fn en_adc_vbat(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_adc_vsys` field of the register.
        ///
        /// Enable system voltage (VSYS pin) ADC channel.
        pub fn en_adc_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `en_adc_ichg` field of the register.
        ///
        /// Enable charge current ADC channel.
        pub fn en_adc_ichg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `en_adc_idchg` field of the register.
        ///
        /// Enable discharge current ADC channel.
        pub fn en_adc_idchg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `en_adc_iin` field of the register.
        ///
        /// Enable input current ADC channel.
        pub fn en_adc_iin(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `en_adc_psys` field of the register.
        ///
        /// Enable system power (PSYS pin) ADC channel.
        pub fn en_adc_psys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `en_adc_vbus` field of the register.
        ///
        /// Enable VBUS voltage ADC channel.
        pub fn en_adc_vbus(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_adc_cmpin` field of the register.
        ///
        /// Enable CMPIN voltage ADC channel.
        pub fn en_adc_cmpin(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `adc_fullscale` field of the register.
        ///
        /// ADC input voltage range for PSYS and CMPIN channels. 0=2.04 V full scale (8 mV/LSB). 1=3.06 V full scale (12 mV/LSB). Not accurate for REGN<6 V application (VBUS and VSYS < 6 V).
        pub fn adc_fullscale(&self) -> super::AdcFullScale {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `adc_start` field of the register.
        ///
        /// Start ADC conversion. Auto-resets to 0 after one-shot update completes.
        pub fn adc_start(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `adc_conv` field of the register.
        ///
        /// ADC conversion mode. 0=One-shot update after ADC_START=1. 1=Continuous update every 1 sec. Each channel conversion time is 25 ms max. Total time is 25 ms times enabled channel count.
        pub fn adc_conv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Write the `en_adc_vbat` field of the register.
        ///
        /// Enable battery voltage (SRN pin) ADC channel.
        pub fn set_en_adc_vbat(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_adc_vsys` field of the register.
        ///
        /// Enable system voltage (VSYS pin) ADC channel.
        pub fn set_en_adc_vsys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_adc_ichg` field of the register.
        ///
        /// Enable charge current ADC channel.
        pub fn set_en_adc_ichg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `en_adc_idchg` field of the register.
        ///
        /// Enable discharge current ADC channel.
        pub fn set_en_adc_idchg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `en_adc_iin` field of the register.
        ///
        /// Enable input current ADC channel.
        pub fn set_en_adc_iin(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `en_adc_psys` field of the register.
        ///
        /// Enable system power (PSYS pin) ADC channel.
        pub fn set_en_adc_psys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `en_adc_vbus` field of the register.
        ///
        /// Enable VBUS voltage ADC channel.
        pub fn set_en_adc_vbus(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_adc_cmpin` field of the register.
        ///
        /// Enable CMPIN voltage ADC channel.
        pub fn set_en_adc_cmpin(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `adc_fullscale` field of the register.
        ///
        /// ADC input voltage range for PSYS and CMPIN channels. 0=2.04 V full scale (8 mV/LSB). 1=3.06 V full scale (12 mV/LSB). Not accurate for REGN<6 V application (VBUS and VSYS < 6 V).
        pub fn set_adc_fullscale(&mut self, value: super::AdcFullScale) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 13, 14, &mut self.bits) };
        }
        ///Write the `adc_start` field of the register.
        ///
        /// Start ADC conversion. Auto-resets to 0 after one-shot update completes.
        pub fn set_adc_start(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 14, 15, &mut self.bits) };
        }
        ///Write the `adc_conv` field of the register.
        ///
        /// ADC conversion mode. 0=One-shot update after ADC_START=1. 1=Continuous update every 1 sec. Each channel conversion time is 25 ms max. Total time is 25 ms times enabled channel count.
        pub fn set_adc_conv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for AdcOption {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcOption> for [u8; 2] {
        fn from(val: AdcOption) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcOption {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcOption");
            d.field("en_adc_vbat", &self.en_adc_vbat());
            d.field("en_adc_vsys", &self.en_adc_vsys());
            d.field("en_adc_ichg", &self.en_adc_ichg());
            d.field("en_adc_idchg", &self.en_adc_idchg());
            d.field("en_adc_iin", &self.en_adc_iin());
            d.field("en_adc_psys", &self.en_adc_psys());
            d.field("en_adc_vbus", &self.en_adc_vbus());
            d.field("en_adc_cmpin", &self.en_adc_cmpin());
            d.field("adc_fullscale", &self.adc_fullscale());
            d.field("adc_start", &self.adc_start());
            d.field("adc_conv", &self.adc_conv());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcOption {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcOption {{ ");
            defmt::write!(f, "en_adc_vbat: {=bool}, ", &self.en_adc_vbat());
            defmt::write!(f, "en_adc_vsys: {=bool}, ", &self.en_adc_vsys());
            defmt::write!(f, "en_adc_ichg: {=bool}, ", &self.en_adc_ichg());
            defmt::write!(f, "en_adc_idchg: {=bool}, ", &self.en_adc_idchg());
            defmt::write!(f, "en_adc_iin: {=bool}, ", &self.en_adc_iin());
            defmt::write!(f, "en_adc_psys: {=bool}, ", &self.en_adc_psys());
            defmt::write!(f, "en_adc_vbus: {=bool}, ", &self.en_adc_vbus());
            defmt::write!(f, "en_adc_cmpin: {=bool}, ", &self.en_adc_cmpin());
            defmt::write!(f, "adc_fullscale: {}, ", &self.adc_fullscale());
            defmt::write!(f, "adc_start: {=bool}, ", &self.adc_start());
            defmt::write!(f, "adc_conv: {=bool}, ", &self.adc_conv());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcOption {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcOption {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcOption {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcOption {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcOption {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcOption {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcOption {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption4 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargeOption4 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargeOption4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [72, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `stat_ptm` field of the register.
        ///
        /// PTM operation status. 0=not in PTM, 1=in PTM.
        pub fn stat_ptm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `stat_idchg_2` field of the register.
        ///
        /// IDCHG2 PROCHOT status. Latched until read by host.
        pub fn stat_idchg_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `pp_idchg_2` field of the register.
        ///
        /// IDCHG2 PROCHOT Profile enable.
        pub fn pp_idchg_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `idchg_th_2` field of the register.
        ///
        /// Battery discharge current limit 2 as percentage of IDCHG_TH1. Setting higher than 32256 mA loses accuracy. Not recommended to set higher than 20 A for 1S OTG boost operation.
        pub fn idchg_th_2(&self) -> super::IdchgTh2Percent {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `idchg_deg_2` field of the register.
        ///
        /// Battery discharge current limit 2 deglitch time (minimum value).
        pub fn idchg_deg_2(&self) -> super::IdchgDeg2Time {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `stat_vbus_vap` field of the register.
        ///
        /// VBUS_VAP PROCHOT status. Latched until read by host.
        pub fn stat_vbus_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `pp_vbus_vap` field of the register.
        ///
        /// VBUS_VAP PROCHOT Profile enable.
        pub fn pp_vbus_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `vsys_uvp_no_hiccup` field of the register.
        ///
        /// Disable VSYS_UVP Hiccup mode. 0=hiccup enabled (charger restarts and latches off after 7 failures in 90 s). 1=hiccup disabled (direct latch-off after 2 ms deglitch).
        pub fn vsys_uvp_no_hiccup(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `en_dither` field of the register.
        ///
        /// Frequency Dither configuration. 0=disabled, 1=+/-2% Fs, 2=+/-4% Fs, 3=+/-6% Fs.
        pub fn en_dither(&self) -> super::DitherConfig {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 13) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vsys_uvp` field of the register.
        ///
        /// VSYS Under Voltage Lock Out threshold. 3-bit, range 2.4-8.0 V, LSB 800 mV. Default 2.4 V (000b). After UVP triggered, charger enters hiccup mode and latches off if restart fails 7 times in 90 s. 000=2.4 V, 001=3.2 V, 010=4.0 V, 011=4.8 V, 100=5.6 V, 101=6.4 V, 110=7.2 V, 111=8.0 V.
        pub fn vsys_uvp(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 16) };
            raw
        }
        ///Write the `pp_idchg_2` field of the register.
        ///
        /// IDCHG2 PROCHOT Profile enable.
        pub fn set_pp_idchg_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `idchg_th_2` field of the register.
        ///
        /// Battery discharge current limit 2 as percentage of IDCHG_TH1. Setting higher than 32256 mA loses accuracy. Not recommended to set higher than 20 A for 1S OTG boost operation.
        pub fn set_idchg_th_2(&mut self, value: super::IdchgTh2Percent) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 6, &mut self.bits) };
        }
        ///Write the `idchg_deg_2` field of the register.
        ///
        /// Battery discharge current limit 2 deglitch time (minimum value).
        pub fn set_idchg_deg_2(&mut self, value: super::IdchgDeg2Time) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 8, &mut self.bits) };
        }
        ///Write the `pp_vbus_vap` field of the register.
        ///
        /// VBUS_VAP PROCHOT Profile enable.
        pub fn set_pp_vbus_vap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `vsys_uvp_no_hiccup` field of the register.
        ///
        /// Disable VSYS_UVP Hiccup mode. 0=hiccup enabled (charger restarts and latches off after 7 failures in 90 s). 1=hiccup disabled (direct latch-off after 2 ms deglitch).
        pub fn set_vsys_uvp_no_hiccup(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `en_dither` field of the register.
        ///
        /// Frequency Dither configuration. 0=disabled, 1=+/-2% Fs, 2=+/-4% Fs, 3=+/-6% Fs.
        pub fn set_en_dither(&mut self, value: super::DitherConfig) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 13, &mut self.bits) };
        }
        ///Write the `vsys_uvp` field of the register.
        ///
        /// VSYS Under Voltage Lock Out threshold. 3-bit, range 2.4-8.0 V, LSB 800 mV. Default 2.4 V (000b). After UVP triggered, charger enters hiccup mode and latches off if restart fails 7 times in 90 s. 000=2.4 V, 001=3.2 V, 010=4.0 V, 011=4.8 V, 100=5.6 V, 101=6.4 V, 110=7.2 V, 111=8.0 V.
        pub fn set_vsys_uvp(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 13, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for ChargeOption4 {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption4> for [u8; 2] {
        fn from(val: ChargeOption4) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption4 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption4");
            d.field("stat_ptm", &self.stat_ptm());
            d.field("stat_idchg_2", &self.stat_idchg_2());
            d.field("pp_idchg_2", &self.pp_idchg_2());
            d.field("idchg_th_2", &self.idchg_th_2());
            d.field("idchg_deg_2", &self.idchg_deg_2());
            d.field("stat_vbus_vap", &self.stat_vbus_vap());
            d.field("pp_vbus_vap", &self.pp_vbus_vap());
            d.field("vsys_uvp_no_hiccup", &self.vsys_uvp_no_hiccup());
            d.field("en_dither", &self.en_dither());
            d.field("vsys_uvp", &self.vsys_uvp());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption4 {{ ");
            defmt::write!(f, "stat_ptm: {=bool}, ", &self.stat_ptm());
            defmt::write!(f, "stat_idchg_2: {=bool}, ", &self.stat_idchg_2());
            defmt::write!(f, "pp_idchg_2: {=bool}, ", &self.pp_idchg_2());
            defmt::write!(f, "idchg_th_2: {}, ", &self.idchg_th_2());
            defmt::write!(f, "idchg_deg_2: {}, ", &self.idchg_deg_2());
            defmt::write!(f, "stat_vbus_vap: {=bool}, ", &self.stat_vbus_vap());
            defmt::write!(f, "pp_vbus_vap: {=bool}, ", &self.pp_vbus_vap());
            defmt::write!(f, "vsys_uvp_no_hiccup: {=bool}, ", &self.vsys_uvp_no_hiccup());
            defmt::write!(f, "en_dither: {}, ", &self.en_dither());
            defmt::write!(f, "vsys_uvp: {=u8}, ", &self.vsys_uvp());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption4 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption4 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption4 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption4 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption4 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption4 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption4 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VminActiveProtection {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for VminActiveProtection {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl VminActiveProtection {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [108, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `en_frs` field of the register.
        ///
        /// Fast Role Swap feature enable. Not recommended to change during OTG operation (disables power stage for ~50 us at 800 kHz). HIZ mode holds higher priority; if EN_HIZ=1, EN_FRS is forced to 0.
        pub fn en_frs(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_vsysth_2_follow_vsysth_1` field of the register.
        ///
        /// Enable internal VSYS_TH2 to follow VSYS_TH1 setting, neglecting VSYS_TH2 register value.
        pub fn en_vsysth_2_follow_vsysth_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `vsys_th_2` field of the register.
        ///
        /// VSYS Threshold 2 to assert STAT_VSYS PROCHOT. 6-bit, fixed 5 us deglitch. Triggers when VSYS pin voltage is below threshold. Fixed DC offset 3.2 V, LSB 100 mV. 2S-4S range 3.2-9.5 V (default 5.9 V, 011011b). 1S range 3.2-3.9 V (default 3.2 V, 000000b).
        pub fn vsys_th_2(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 8) };
            raw
        }
        ///Read the `vbus_vap_th` field of the register.
        ///
        /// VAP mode VBUS PROCHOT trigger voltage threshold. 7-bit, fixed DC offset 3.2 V, LSB 100 mV, range 3.2-15.9 V. Default 3.2 V (0000000b).
        pub fn vbus_vap_th(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 16) };
            raw
        }
        ///Write the `en_frs` field of the register.
        ///
        /// Fast Role Swap feature enable. Not recommended to change during OTG operation (disables power stage for ~50 us at 800 kHz). HIZ mode holds higher priority; if EN_HIZ=1, EN_FRS is forced to 0.
        pub fn set_en_frs(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_vsysth_2_follow_vsysth_1` field of the register.
        ///
        /// Enable internal VSYS_TH2 to follow VSYS_TH1 setting, neglecting VSYS_TH2 register value.
        pub fn set_en_vsysth_2_follow_vsysth_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `vsys_th_2` field of the register.
        ///
        /// VSYS Threshold 2 to assert STAT_VSYS PROCHOT. 6-bit, fixed 5 us deglitch. Triggers when VSYS pin voltage is below threshold. Fixed DC offset 3.2 V, LSB 100 mV. 2S-4S range 3.2-9.5 V (default 5.9 V, 011011b). 1S range 3.2-3.9 V (default 3.2 V, 000000b).
        pub fn set_vsys_th_2(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 8, &mut self.bits) };
        }
        ///Write the `vbus_vap_th` field of the register.
        ///
        /// VAP mode VBUS PROCHOT trigger voltage threshold. 7-bit, fixed DC offset 3.2 V, LSB 100 mV, range 3.2-15.9 V. Default 3.2 V (0000000b).
        pub fn set_vbus_vap_th(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for VminActiveProtection {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<VminActiveProtection> for [u8; 2] {
        fn from(val: VminActiveProtection) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VminActiveProtection {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VminActiveProtection");
            d.field("en_frs", &self.en_frs());
            d.field("en_vsysth_2_follow_vsysth_1", &self.en_vsysth_2_follow_vsysth_1());
            d.field("vsys_th_2", &self.vsys_th_2());
            d.field("vbus_vap_th", &self.vbus_vap_th());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for VminActiveProtection {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VminActiveProtection {{ ");
            defmt::write!(f, "en_frs: {=bool}, ", &self.en_frs());
            defmt::write!(
                f,
                "en_vsysth_2_follow_vsysth_1: {=bool}, ",
                &self.en_vsysth_2_follow_vsysth_1()
            );
            defmt::write!(f, "vsys_th_2: {=u8}, ", &self.vsys_th_2());
            defmt::write!(f, "vbus_vap_th: {=u8}, ", &self.vbus_vap_th());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for VminActiveProtection {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VminActiveProtection {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VminActiveProtection {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VminActiveProtection {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VminActiveProtection {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VminActiveProtection {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VminActiveProtection {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Enum containing all possible field set types
    pub enum FieldSetValue {
        ChargeOption0(ChargeOption0),
        ChargeCurrent(ChargeCurrent),
        ChargeVoltage(ChargeVoltage),
        OtgVoltage(OtgVoltage),
        OtgCurrent(OtgCurrent),
        InputVoltage(InputVoltage),
        VsysMin(VsysMin),
        IinHost(IinHost),
        ChargerStatus(ChargerStatus),
        ProchotStatus(ProchotStatus),
        IinDpm(IinDpm),
        AdcVbusPsys(AdcVbusPsys),
        AdcIbat(AdcIbat),
        AdcIinCmpin(AdcIinCmpin),
        AdcVsysVbat(AdcVsysVbat),
        ManufacturerId(ManufacturerId),
        DeviceId(DeviceId),
        ChargeOption1(ChargeOption1),
        ChargeOption2(ChargeOption2),
        ChargeOption3(ChargeOption3),
        ProchotOption0(ProchotOption0),
        ProchotOption1(ProchotOption1),
        AdcOption(AdcOption),
        ChargeOption4(ChargeOption4),
        VminActiveProtection(VminActiveProtection),
    }
    impl core::fmt::Debug for FieldSetValue {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::ChargeOption0(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeCurrent(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeVoltage(val) => core::fmt::Debug::fmt(val, f),
                Self::OtgVoltage(val) => core::fmt::Debug::fmt(val, f),
                Self::OtgCurrent(val) => core::fmt::Debug::fmt(val, f),
                Self::InputVoltage(val) => core::fmt::Debug::fmt(val, f),
                Self::VsysMin(val) => core::fmt::Debug::fmt(val, f),
                Self::IinHost(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargerStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::IinDpm(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcVbusPsys(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcIbat(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcIinCmpin(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcVsysVbat(val) => core::fmt::Debug::fmt(val, f),
                Self::ManufacturerId(val) => core::fmt::Debug::fmt(val, f),
                Self::DeviceId(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption1(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption2(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption3(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotOption0(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotOption1(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcOption(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption4(val) => core::fmt::Debug::fmt(val, f),
                Self::VminActiveProtection(val) => core::fmt::Debug::fmt(val, f),
                #[allow(unreachable_patterns)]
                _ => unreachable!(),
            }
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for FieldSetValue {
        fn format(&self, f: defmt::Formatter) {
            match self {
                Self::ChargeOption0(val) => defmt::Format::format(val, f),
                Self::ChargeCurrent(val) => defmt::Format::format(val, f),
                Self::ChargeVoltage(val) => defmt::Format::format(val, f),
                Self::OtgVoltage(val) => defmt::Format::format(val, f),
                Self::OtgCurrent(val) => defmt::Format::format(val, f),
                Self::InputVoltage(val) => defmt::Format::format(val, f),
                Self::VsysMin(val) => defmt::Format::format(val, f),
                Self::IinHost(val) => defmt::Format::format(val, f),
                Self::ChargerStatus(val) => defmt::Format::format(val, f),
                Self::ProchotStatus(val) => defmt::Format::format(val, f),
                Self::IinDpm(val) => defmt::Format::format(val, f),
                Self::AdcVbusPsys(val) => defmt::Format::format(val, f),
                Self::AdcIbat(val) => defmt::Format::format(val, f),
                Self::AdcIinCmpin(val) => defmt::Format::format(val, f),
                Self::AdcVsysVbat(val) => defmt::Format::format(val, f),
                Self::ManufacturerId(val) => defmt::Format::format(val, f),
                Self::DeviceId(val) => defmt::Format::format(val, f),
                Self::ChargeOption1(val) => defmt::Format::format(val, f),
                Self::ChargeOption2(val) => defmt::Format::format(val, f),
                Self::ChargeOption3(val) => defmt::Format::format(val, f),
                Self::ProchotOption0(val) => defmt::Format::format(val, f),
                Self::ProchotOption1(val) => defmt::Format::format(val, f),
                Self::AdcOption(val) => defmt::Format::format(val, f),
                Self::ChargeOption4(val) => defmt::Format::format(val, f),
                Self::VminActiveProtection(val) => defmt::Format::format(val, f),
            }
        }
    }
    impl From<ChargeOption0> for FieldSetValue {
        fn from(val: ChargeOption0) -> Self {
            Self::ChargeOption0(val)
        }
    }
    impl From<ChargeCurrent> for FieldSetValue {
        fn from(val: ChargeCurrent) -> Self {
            Self::ChargeCurrent(val)
        }
    }
    impl From<ChargeVoltage> for FieldSetValue {
        fn from(val: ChargeVoltage) -> Self {
            Self::ChargeVoltage(val)
        }
    }
    impl From<OtgVoltage> for FieldSetValue {
        fn from(val: OtgVoltage) -> Self {
            Self::OtgVoltage(val)
        }
    }
    impl From<OtgCurrent> for FieldSetValue {
        fn from(val: OtgCurrent) -> Self {
            Self::OtgCurrent(val)
        }
    }
    impl From<InputVoltage> for FieldSetValue {
        fn from(val: InputVoltage) -> Self {
            Self::InputVoltage(val)
        }
    }
    impl From<VsysMin> for FieldSetValue {
        fn from(val: VsysMin) -> Self {
            Self::VsysMin(val)
        }
    }
    impl From<IinHost> for FieldSetValue {
        fn from(val: IinHost) -> Self {
            Self::IinHost(val)
        }
    }
    impl From<ChargerStatus> for FieldSetValue {
        fn from(val: ChargerStatus) -> Self {
            Self::ChargerStatus(val)
        }
    }
    impl From<ProchotStatus> for FieldSetValue {
        fn from(val: ProchotStatus) -> Self {
            Self::ProchotStatus(val)
        }
    }
    impl From<IinDpm> for FieldSetValue {
        fn from(val: IinDpm) -> Self {
            Self::IinDpm(val)
        }
    }
    impl From<AdcVbusPsys> for FieldSetValue {
        fn from(val: AdcVbusPsys) -> Self {
            Self::AdcVbusPsys(val)
        }
    }
    impl From<AdcIbat> for FieldSetValue {
        fn from(val: AdcIbat) -> Self {
            Self::AdcIbat(val)
        }
    }
    impl From<AdcIinCmpin> for FieldSetValue {
        fn from(val: AdcIinCmpin) -> Self {
            Self::AdcIinCmpin(val)
        }
    }
    impl From<AdcVsysVbat> for FieldSetValue {
        fn from(val: AdcVsysVbat) -> Self {
            Self::AdcVsysVbat(val)
        }
    }
    impl From<ManufacturerId> for FieldSetValue {
        fn from(val: ManufacturerId) -> Self {
            Self::ManufacturerId(val)
        }
    }
    impl From<DeviceId> for FieldSetValue {
        fn from(val: DeviceId) -> Self {
            Self::DeviceId(val)
        }
    }
    impl From<ChargeOption1> for FieldSetValue {
        fn from(val: ChargeOption1) -> Self {
            Self::ChargeOption1(val)
        }
    }
    impl From<ChargeOption2> for FieldSetValue {
        fn from(val: ChargeOption2) -> Self {
            Self::ChargeOption2(val)
        }
    }
    impl From<ChargeOption3> for FieldSetValue {
        fn from(val: ChargeOption3) -> Self {
            Self::ChargeOption3(val)
        }
    }
    impl From<ProchotOption0> for FieldSetValue {
        fn from(val: ProchotOption0) -> Self {
            Self::ProchotOption0(val)
        }
    }
    impl From<ProchotOption1> for FieldSetValue {
        fn from(val: ProchotOption1) -> Self {
            Self::ProchotOption1(val)
        }
    }
    impl From<AdcOption> for FieldSetValue {
        fn from(val: AdcOption) -> Self {
            Self::AdcOption(val)
        }
    }
    impl From<ChargeOption4> for FieldSetValue {
        fn from(val: ChargeOption4) -> Self {
            Self::ChargeOption4(val)
        }
    }
    impl From<VminActiveProtection> for FieldSetValue {
        fn from(val: VminActiveProtection) -> Self {
            Self::VminActiveProtection(val)
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IbatGain {
    EightX = 0,
    SixteenX = 1,
}
impl core::convert::TryFrom<u8> for IbatGain {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::EightX),
            1 => Ok(Self::SixteenX),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IbatGain",
            }),
        }
    }
}
impl From<IbatGain> for u8 {
    fn from(val: IbatGain) -> Self {
        match val {
            IbatGain::EightX => 0,
            IbatGain::SixteenX => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IadptGain {
    TwentyX = 0,
    FortyX = 1,
}
impl core::convert::TryFrom<u8> for IadptGain {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TwentyX),
            1 => Ok(Self::FortyX),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IadptGain",
            }),
        }
    }
}
impl From<IadptGain> for u8 {
    fn from(val: IadptGain) -> Self {
        match val {
            IadptGain::TwentyX => 0,
            IadptGain::FortyX => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum SwitchingFreq {
    TwelveHundredkHz = 0,
    EightHundredkHz = 1,
}
impl core::convert::TryFrom<u8> for SwitchingFreq {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TwelveHundredkHz),
            1 => Ok(Self::EightHundredkHz),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "SwitchingFreq",
            }),
        }
    }
}
impl From<SwitchingFreq> for u8 {
    fn from(val: SwitchingFreq) -> Self {
        match val {
            SwitchingFreq::TwelveHundredkHz => 0,
            SwitchingFreq::EightHundredkHz => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum WatchdogTimerAdj {
    Disabled = 0,
    FiveSeconds = 1,
    EightyEightSeconds = 2,
    OneHundredSeventyFiveSeconds = 3,
}
impl core::convert::TryFrom<u8> for WatchdogTimerAdj {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::FiveSeconds),
            2 => Ok(Self::EightyEightSeconds),
            3 => Ok(Self::OneHundredSeventyFiveSeconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "WatchdogTimerAdj",
            }),
        }
    }
}
impl From<WatchdogTimerAdj> for u8 {
    fn from(val: WatchdogTimerAdj) -> Self {
        match val {
            WatchdogTimerAdj::Disabled => 0,
            WatchdogTimerAdj::FiveSeconds => 1,
            WatchdogTimerAdj::EightyEightSeconds => 2,
            WatchdogTimerAdj::OneHundredSeventyFiveSeconds => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ProchotPulseWidth {
    OneHundredMicroseconds = 0,
    OneMillisecond = 1,
    FiveMilliseconds = 2,
    TenMilliseconds = 3,
}
impl core::convert::TryFrom<u8> for ProchotPulseWidth {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneHundredMicroseconds),
            1 => Ok(Self::OneMillisecond),
            2 => Ok(Self::FiveMilliseconds),
            3 => Ok(Self::TenMilliseconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ProchotPulseWidth",
            }),
        }
    }
}
impl From<ProchotPulseWidth> for u8 {
    fn from(val: ProchotPulseWidth) -> Self {
        match val {
            ProchotPulseWidth::OneHundredMicroseconds => 0,
            ProchotPulseWidth::OneMillisecond => 1,
            ProchotPulseWidth::FiveMilliseconds => 2,
            ProchotPulseWidth::TenMilliseconds => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ComparatorDeglitchTime {
    FiveMicroseconds = 0,
    TwoMilliseconds = 1,
    TwentyMilliseconds = 2,
    FiveSeconds = 3,
}
impl core::convert::TryFrom<u8> for ComparatorDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::FiveMicroseconds),
            1 => Ok(Self::TwoMilliseconds),
            2 => Ok(Self::TwentyMilliseconds),
            3 => Ok(Self::FiveSeconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ComparatorDeglitchTime",
            }),
        }
    }
}
impl From<ComparatorDeglitchTime> for u8 {
    fn from(val: ComparatorDeglitchTime) -> Self {
        match val {
            ComparatorDeglitchTime::FiveMicroseconds => 0,
            ComparatorDeglitchTime::TwoMilliseconds => 1,
            ComparatorDeglitchTime::TwentyMilliseconds => 2,
            ComparatorDeglitchTime::FiveSeconds => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum PsysGain {
    ZeroPointTwoFiveuAperW = 0,
    OneuAperW = 1,
}
impl core::convert::TryFrom<u8> for PsysGain {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ZeroPointTwoFiveuAperW),
            1 => Ok(Self::OneuAperW),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PsysGain",
            }),
        }
    }
}
impl From<PsysGain> for u8 {
    fn from(val: PsysGain) -> Self {
        match val {
            PsysGain::ZeroPointTwoFiveuAperW => 0,
            PsysGain::OneuAperW => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargeSenseResistor {
    TenMilliOhm = 0,
    FiveMilliOhm = 1,
}
impl core::convert::TryFrom<u8> for ChargeSenseResistor {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TenMilliOhm),
            1 => Ok(Self::FiveMilliOhm),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ChargeSenseResistor",
            }),
        }
    }
}
impl From<ChargeSenseResistor> for u8 {
    fn from(val: ChargeSenseResistor) -> Self {
        match val {
            ChargeSenseResistor::TenMilliOhm => 0,
            ChargeSenseResistor::FiveMilliOhm => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum InputSenseResistor {
    TenMilliOhm = 0,
    FiveMilliOhm = 1,
}
impl core::convert::TryFrom<u8> for InputSenseResistor {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TenMilliOhm),
            1 => Ok(Self::FiveMilliOhm),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "InputSenseResistor",
            }),
        }
    }
}
impl From<InputSenseResistor> for u8 {
    fn from(val: InputSenseResistor) -> Self {
        match val {
            InputSenseResistor::TenMilliOhm => 0,
            InputSenseResistor::FiveMilliOhm => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum PsysConfig {
    PbusAndPbat = 0,
    Pbus = 1,
    Reserved = 2,
    Disabled = 3,
}
impl core::convert::TryFrom<u8> for PsysConfig {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::PbusAndPbat),
            1 => Ok(Self::Pbus),
            2 => Ok(Self::Reserved),
            3 => Ok(Self::Disabled),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PsysConfig",
            }),
        }
    }
}
impl From<PsysConfig> for u8 {
    fn from(val: PsysConfig) -> Self {
        match val {
            PsysConfig::PbusAndPbat => 0,
            PsysConfig::Pbus => 1,
            PsysConfig::Reserved => 2,
            PsysConfig::Disabled => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum BatocThreshold {
    OneHundredThirtyThreePercent = 0,
    TwoHundredPercent = 1,
}
impl core::convert::TryFrom<u8> for BatocThreshold {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneHundredThirtyThreePercent),
            1 => Ok(Self::TwoHundredPercent),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "BatocThreshold",
            }),
        }
    }
}
impl From<BatocThreshold> for u8 {
    fn from(val: BatocThreshold) -> Self {
        match val {
            BatocThreshold::OneHundredThirtyThreePercent => 0,
            BatocThreshold::TwoHundredPercent => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum AcocThreshold {
    OneHundredThirtyThreePercent = 0,
    TwoHundredPercent = 1,
}
impl core::convert::TryFrom<u8> for AcocThreshold {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneHundredThirtyThreePercent),
            1 => Ok(Self::TwoHundredPercent),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AcocThreshold",
            }),
        }
    }
}
impl From<AcocThreshold> for u8 {
    fn from(val: AcocThreshold) -> Self {
        match val {
            AcocThreshold::OneHundredThirtyThreePercent => 0,
            AcocThreshold::TwoHundredPercent => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum AcxOcpThreshold {
    High = 0,
    Low = 1,
}
impl core::convert::TryFrom<u8> for AcxOcpThreshold {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::High),
            1 => Ok(Self::Low),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AcxOcpThreshold",
            }),
        }
    }
}
impl From<AcxOcpThreshold> for u8 {
    fn from(val: AcxOcpThreshold) -> Self {
        match val {
            AcxOcpThreshold::High => 0,
            AcxOcpThreshold::Low => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Q2OcpThreshold {
    TwoHundredTenMilliVolts = 0,
    OneHundredFiftyMilliVolts = 1,
}
impl core::convert::TryFrom<u8> for Q2OcpThreshold {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TwoHundredTenMilliVolts),
            1 => Ok(Self::OneHundredFiftyMilliVolts),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Q2OcpThreshold",
            }),
        }
    }
}
impl From<Q2OcpThreshold> for u8 {
    fn from(val: Q2OcpThreshold) -> Self {
        match val {
            Q2OcpThreshold::TwoHundredTenMilliVolts => 0,
            Q2OcpThreshold::OneHundredFiftyMilliVolts => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IBatPinSelect {
    DischargeCurrent = 0,
    ChargeCurrent = 1,
}
impl core::convert::TryFrom<u8> for IBatPinSelect {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::DischargeCurrent),
            1 => Ok(Self::ChargeCurrent),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IBatPinSelect",
            }),
        }
    }
}
impl From<IBatPinSelect> for u8 {
    fn from(val: IBatPinSelect) -> Self {
        match val {
            IBatPinSelect::DischargeCurrent => 0,
            IBatPinSelect::ChargeCurrent => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum PkpwrTmax {
    TwentyMilliseconds = 0,
    FortyMilliseconds = 1,
    EightyMilliseconds = 2,
    OneSecond = 3,
}
impl core::convert::TryFrom<u8> for PkpwrTmax {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TwentyMilliseconds),
            1 => Ok(Self::FortyMilliseconds),
            2 => Ok(Self::EightyMilliseconds),
            3 => Ok(Self::OneSecond),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PkpwrTmax",
            }),
        }
    }
}
impl From<PkpwrTmax> for u8 {
    fn from(val: PkpwrTmax) -> Self {
        match val {
            PkpwrTmax::TwentyMilliseconds => 0,
            PkpwrTmax::FortyMilliseconds => 1,
            PkpwrTmax::EightyMilliseconds => 2,
            PkpwrTmax::OneSecond => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum PkpwrTovldDeg {
    OneMillisecond = 0,
    TwoMilliseconds = 1,
    FiveMilliseconds = 2,
    TenMilliseconds = 3,
}
impl core::convert::TryFrom<u8> for PkpwrTovldDeg {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneMillisecond),
            1 => Ok(Self::TwoMilliseconds),
            2 => Ok(Self::FiveMilliseconds),
            3 => Ok(Self::TenMilliseconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PkpwrTovldDeg",
            }),
        }
    }
}
impl From<PkpwrTovldDeg> for u8 {
    fn from(val: PkpwrTovldDeg) -> Self {
        match val {
            PkpwrTovldDeg::OneMillisecond => 0,
            PkpwrTovldDeg::TwoMilliseconds => 1,
            PkpwrTovldDeg::FiveMilliseconds => 2,
            PkpwrTovldDeg::TenMilliseconds => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum PsysOtgIdchg {
    BatteryMinusOtg = 0,
    BatteryOnly = 1,
}
impl core::convert::TryFrom<u8> for PsysOtgIdchg {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::BatteryMinusOtg),
            1 => Ok(Self::BatteryOnly),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PsysOtgIdchg",
            }),
        }
    }
}
impl From<PsysOtgIdchg> for u8 {
    fn from(val: PsysOtgIdchg) -> Self {
        match val {
            PsysOtgIdchg::BatteryMinusOtg => 0,
            PsysOtgIdchg::BatteryOnly => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum InductorAvgCurrentClamp {
    SixAmps = 0,
    TenAmps = 1,
    FifteenAmps = 2,
    Disabled = 3,
}
impl core::convert::TryFrom<u8> for InductorAvgCurrentClamp {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::SixAmps),
            1 => Ok(Self::TenAmps),
            2 => Ok(Self::FifteenAmps),
            3 => Ok(Self::Disabled),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "InductorAvgCurrentClamp",
            }),
        }
    }
}
impl From<InductorAvgCurrentClamp> for u8 {
    fn from(val: InductorAvgCurrentClamp) -> Self {
        match val {
            InductorAvgCurrentClamp::SixAmps => 0,
            InductorAvgCurrentClamp::TenAmps => 1,
            InductorAvgCurrentClamp::FifteenAmps => 2,
            InductorAvgCurrentClamp::Disabled => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum InomDeglitchTime {
    OneMillisecond = 0,
    SixtyMilliseconds = 1,
}
impl core::convert::TryFrom<u8> for InomDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneMillisecond),
            1 => Ok(Self::SixtyMilliseconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "InomDeglitchTime",
            }),
        }
    }
}
impl From<InomDeglitchTime> for u8 {
    fn from(val: InomDeglitchTime) -> Self {
        match val {
            InomDeglitchTime::OneMillisecond => 0,
            InomDeglitchTime::SixtyMilliseconds => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ProchotVindpmThreshold {
    EightyThreePercent = 0,
    NinetyOnePercent = 1,
}
impl core::convert::TryFrom<u8> for ProchotVindpmThreshold {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::EightyThreePercent),
            1 => Ok(Self::NinetyOnePercent),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ProchotVindpmThreshold",
            }),
        }
    }
}
impl From<ProchotVindpmThreshold> for u8 {
    fn from(val: ProchotVindpmThreshold) -> Self {
        match val {
            ProchotVindpmThreshold::EightyThreePercent => 0,
            ProchotVindpmThreshold::NinetyOnePercent => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IcritDeglitchTime {
    FifteenMicroseconds = 0,
    OneHundredMicroseconds = 1,
    FourHundredMicroseconds = 2,
    EightHundredMicroseconds = 3,
}
impl core::convert::TryFrom<u8> for IcritDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::FifteenMicroseconds),
            1 => Ok(Self::OneHundredMicroseconds),
            2 => Ok(Self::FourHundredMicroseconds),
            3 => Ok(Self::EightHundredMicroseconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IcritDeglitchTime",
            }),
        }
    }
}
impl From<IcritDeglitchTime> for u8 {
    fn from(val: IcritDeglitchTime) -> Self {
        match val {
            IcritDeglitchTime::FifteenMicroseconds => 0,
            IcritDeglitchTime::OneHundredMicroseconds => 1,
            IcritDeglitchTime::FourHundredMicroseconds => 2,
            IcritDeglitchTime::EightHundredMicroseconds => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IdchgDeg1Time {
    SeventyEightMilliseconds = 0,
    OnePointTwoFiveSeconds = 1,
    FiveSeconds = 2,
    TwentySeconds = 3,
}
impl core::convert::TryFrom<u8> for IdchgDeg1Time {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::SeventyEightMilliseconds),
            1 => Ok(Self::OnePointTwoFiveSeconds),
            2 => Ok(Self::FiveSeconds),
            3 => Ok(Self::TwentySeconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IdchgDeg1Time",
            }),
        }
    }
}
impl From<IdchgDeg1Time> for u8 {
    fn from(val: IdchgDeg1Time) -> Self {
        match val {
            IdchgDeg1Time::SeventyEightMilliseconds => 0,
            IdchgDeg1Time::OnePointTwoFiveSeconds => 1,
            IdchgDeg1Time::FiveSeconds => 2,
            IdchgDeg1Time::TwentySeconds => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum AdcFullScale {
    TwoPointZeroFourVolts = 0,
    ThreePointZeroSixVolts = 1,
}
impl core::convert::TryFrom<u8> for AdcFullScale {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TwoPointZeroFourVolts),
            1 => Ok(Self::ThreePointZeroSixVolts),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AdcFullScale",
            }),
        }
    }
}
impl From<AdcFullScale> for u8 {
    fn from(val: AdcFullScale) -> Self {
        match val {
            AdcFullScale::TwoPointZeroFourVolts => 0,
            AdcFullScale::ThreePointZeroSixVolts => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IdchgTh2Percent {
    OneHundredTwentyFivePercent = 0,
    OneHundredFiftyPercent = 1,
    OneHundredSeventyFivePercent = 2,
    TwoHundredPercent = 3,
    TwoHundredFiftyPercent = 4,
    ThreeHundredPercent = 5,
    ThreeHundredFiftyPercent = 6,
    FourHundredPercent = 7,
}
impl core::convert::TryFrom<u8> for IdchgTh2Percent {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneHundredTwentyFivePercent),
            1 => Ok(Self::OneHundredFiftyPercent),
            2 => Ok(Self::OneHundredSeventyFivePercent),
            3 => Ok(Self::TwoHundredPercent),
            4 => Ok(Self::TwoHundredFiftyPercent),
            5 => Ok(Self::ThreeHundredPercent),
            6 => Ok(Self::ThreeHundredFiftyPercent),
            7 => Ok(Self::FourHundredPercent),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IdchgTh2Percent",
            }),
        }
    }
}
impl From<IdchgTh2Percent> for u8 {
    fn from(val: IdchgTh2Percent) -> Self {
        match val {
            IdchgTh2Percent::OneHundredTwentyFivePercent => 0,
            IdchgTh2Percent::OneHundredFiftyPercent => 1,
            IdchgTh2Percent::OneHundredSeventyFivePercent => 2,
            IdchgTh2Percent::TwoHundredPercent => 3,
            IdchgTh2Percent::TwoHundredFiftyPercent => 4,
            IdchgTh2Percent::ThreeHundredPercent => 5,
            IdchgTh2Percent::ThreeHundredFiftyPercent => 6,
            IdchgTh2Percent::FourHundredPercent => 7,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IdchgDeg2Time {
    OneHundredMicroseconds = 0,
    OnePointSixMilliseconds = 1,
    SixMilliseconds = 2,
    TwelveMilliseconds = 3,
}
impl core::convert::TryFrom<u8> for IdchgDeg2Time {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneHundredMicroseconds),
            1 => Ok(Self::OnePointSixMilliseconds),
            2 => Ok(Self::SixMilliseconds),
            3 => Ok(Self::TwelveMilliseconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IdchgDeg2Time",
            }),
        }
    }
}
impl From<IdchgDeg2Time> for u8 {
    fn from(val: IdchgDeg2Time) -> Self {
        match val {
            IdchgDeg2Time::OneHundredMicroseconds => 0,
            IdchgDeg2Time::OnePointSixMilliseconds => 1,
            IdchgDeg2Time::SixMilliseconds => 2,
            IdchgDeg2Time::TwelveMilliseconds => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum DitherConfig {
    Disabled = 0,
    OneX = 1,
    TwoX = 2,
    ThreeX = 3,
}
impl core::convert::TryFrom<u8> for DitherConfig {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::OneX),
            2 => Ok(Self::TwoX),
            3 => Ok(Self::ThreeX),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "DitherConfig",
            }),
        }
    }
}
impl From<DitherConfig> for u8 {
    fn from(val: DitherConfig) -> Self {
        match val {
            DitherConfig::Disabled => 0,
            DitherConfig::OneX => 1,
            DitherConfig::TwoX => 2,
            DitherConfig::ThreeX => 3,
        }
    }
}
