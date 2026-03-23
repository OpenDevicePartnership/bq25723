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
        let reg = self.charge_option_0_a().read()?;
        callback(0 + 0 * 0, "charge_option_0_a", reg.into());
        let reg = self.charge_option_0_b().read()?;
        callback(1 + 0 * 0, "charge_option_0_b", reg.into());
        let reg = self.charge_current().read()?;
        callback(2 + 0 * 0, "charge_current", reg.into());
        let reg = self.charge_voltage().read()?;
        callback(4 + 0 * 0, "charge_voltage", reg.into());
        let reg = self.charger_status_0_a().read()?;
        callback(32 + 0 * 0, "charger_status_0_a", reg.into());
        let reg = self.charger_status_0_b().read()?;
        callback(33 + 0 * 0, "charger_status_0_b", reg.into());
        let reg = self.prochot_status_reg_a().read()?;
        callback(34 + 0 * 0, "prochot_status_reg_a", reg.into());
        let reg = self.prochot_status_reg_b().read()?;
        callback(35 + 0 * 0, "prochot_status_reg_b", reg.into());
        let reg = self.iin_dpm().read()?;
        callback(36 + 0 * 0, "iin_dpm", reg.into());
        let reg = self.adc_psys().read()?;
        callback(38 + 0 * 0, "adc_psys", reg.into());
        let reg = self.adc_vbus().read()?;
        callback(39 + 0 * 0, "adc_vbus", reg.into());
        let reg = self.adc_ibat_dchrg().read()?;
        callback(40 + 0 * 0, "adc_ibat_dchrg", reg.into());
        let reg = self.adc_ibat_chrg().read()?;
        callback(41 + 0 * 0, "adc_ibat_chrg", reg.into());
        let reg = self.adc_cmpin().read()?;
        callback(42 + 0 * 0, "adc_cmpin", reg.into());
        let reg = self.adc_vbat().read()?;
        callback(44 + 0 * 0, "adc_vbat", reg.into());
        let reg = self.adc_vsys().read()?;
        callback(45 + 0 * 0, "adc_vsys", reg.into());
        let reg = self.charge_option_1_a().read()?;
        callback(48 + 0 * 0, "charge_option_1_a", reg.into());
        let reg = self.charge_option_1_b().read()?;
        callback(49 + 0 * 0, "charge_option_1_b", reg.into());
        let reg = self.charge_option_2_a().read()?;
        callback(50 + 0 * 0, "charge_option_2_a", reg.into());
        let reg = self.charge_option_2_b().read()?;
        callback(51 + 0 * 0, "charge_option_2_b", reg.into());
        let reg = self.charge_option_3_a().read()?;
        callback(52 + 0 * 0, "charge_option_3_a", reg.into());
        let reg = self.charge_option_3_b().read()?;
        callback(53 + 0 * 0, "charge_option_3_b", reg.into());
        let reg = self.prochot_option_0_a().read()?;
        callback(54 + 0 * 0, "prochot_option_0_a", reg.into());
        let reg = self.prochot_option_0_b().read()?;
        callback(55 + 0 * 0, "prochot_option_0_b", reg.into());
        let reg = self.prochot_option_1_a().read()?;
        callback(56 + 0 * 0, "prochot_option_1_a", reg.into());
        let reg = self.prochot_option_1_b().read()?;
        callback(57 + 0 * 0, "prochot_option_1_b", reg.into());
        let reg = self.adc_option_a().read()?;
        callback(58 + 0 * 0, "adc_option_a", reg.into());
        let reg = self.adc_option_b().read()?;
        callback(59 + 0 * 0, "adc_option_b", reg.into());
        let reg = self.charge_option_4_a().read()?;
        callback(60 + 0 * 0, "charge_option_4_a", reg.into());
        let reg = self.charge_option_4_b().read()?;
        callback(61 + 0 * 0, "charge_option_4_b", reg.into());
        let reg = self.vmin_active_protection_a().read()?;
        callback(62 + 0 * 0, "vmin_active_protection_a", reg.into());
        let reg = self.vmin_active_protection_b().read()?;
        callback(63 + 0 * 0, "vmin_active_protection_b", reg.into());
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
        let reg = self.manufacture_id().read()?;
        callback(46 + 0 * 0, "manufacture_id", reg.into());
        let reg = self.device_id().read()?;
        callback(47 + 0 * 0, "device_id", reg.into());
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
        let reg = self.charge_option_0_a().read_async().await?;
        callback(0 + 0 * 0, "charge_option_0_a", reg.into());
        let reg = self.charge_option_0_b().read_async().await?;
        callback(1 + 0 * 0, "charge_option_0_b", reg.into());
        let reg = self.charge_current().read_async().await?;
        callback(2 + 0 * 0, "charge_current", reg.into());
        let reg = self.charge_voltage().read_async().await?;
        callback(4 + 0 * 0, "charge_voltage", reg.into());
        let reg = self.charger_status_0_a().read_async().await?;
        callback(32 + 0 * 0, "charger_status_0_a", reg.into());
        let reg = self.charger_status_0_b().read_async().await?;
        callback(33 + 0 * 0, "charger_status_0_b", reg.into());
        let reg = self.prochot_status_reg_a().read_async().await?;
        callback(34 + 0 * 0, "prochot_status_reg_a", reg.into());
        let reg = self.prochot_status_reg_b().read_async().await?;
        callback(35 + 0 * 0, "prochot_status_reg_b", reg.into());
        let reg = self.iin_dpm().read_async().await?;
        callback(36 + 0 * 0, "iin_dpm", reg.into());
        let reg = self.adc_psys().read_async().await?;
        callback(38 + 0 * 0, "adc_psys", reg.into());
        let reg = self.adc_vbus().read_async().await?;
        callback(39 + 0 * 0, "adc_vbus", reg.into());
        let reg = self.adc_ibat_dchrg().read_async().await?;
        callback(40 + 0 * 0, "adc_ibat_dchrg", reg.into());
        let reg = self.adc_ibat_chrg().read_async().await?;
        callback(41 + 0 * 0, "adc_ibat_chrg", reg.into());
        let reg = self.adc_cmpin().read_async().await?;
        callback(42 + 0 * 0, "adc_cmpin", reg.into());
        let reg = self.adc_vbat().read_async().await?;
        callback(44 + 0 * 0, "adc_vbat", reg.into());
        let reg = self.adc_vsys().read_async().await?;
        callback(45 + 0 * 0, "adc_vsys", reg.into());
        let reg = self.charge_option_1_a().read_async().await?;
        callback(48 + 0 * 0, "charge_option_1_a", reg.into());
        let reg = self.charge_option_1_b().read_async().await?;
        callback(49 + 0 * 0, "charge_option_1_b", reg.into());
        let reg = self.charge_option_2_a().read_async().await?;
        callback(50 + 0 * 0, "charge_option_2_a", reg.into());
        let reg = self.charge_option_2_b().read_async().await?;
        callback(51 + 0 * 0, "charge_option_2_b", reg.into());
        let reg = self.charge_option_3_a().read_async().await?;
        callback(52 + 0 * 0, "charge_option_3_a", reg.into());
        let reg = self.charge_option_3_b().read_async().await?;
        callback(53 + 0 * 0, "charge_option_3_b", reg.into());
        let reg = self.prochot_option_0_a().read_async().await?;
        callback(54 + 0 * 0, "prochot_option_0_a", reg.into());
        let reg = self.prochot_option_0_b().read_async().await?;
        callback(55 + 0 * 0, "prochot_option_0_b", reg.into());
        let reg = self.prochot_option_1_a().read_async().await?;
        callback(56 + 0 * 0, "prochot_option_1_a", reg.into());
        let reg = self.prochot_option_1_b().read_async().await?;
        callback(57 + 0 * 0, "prochot_option_1_b", reg.into());
        let reg = self.adc_option_a().read_async().await?;
        callback(58 + 0 * 0, "adc_option_a", reg.into());
        let reg = self.adc_option_b().read_async().await?;
        callback(59 + 0 * 0, "adc_option_b", reg.into());
        let reg = self.charge_option_4_a().read_async().await?;
        callback(60 + 0 * 0, "charge_option_4_a", reg.into());
        let reg = self.charge_option_4_b().read_async().await?;
        callback(61 + 0 * 0, "charge_option_4_b", reg.into());
        let reg = self.vmin_active_protection_a().read_async().await?;
        callback(62 + 0 * 0, "vmin_active_protection_a", reg.into());
        let reg = self.vmin_active_protection_b().read_async().await?;
        callback(63 + 0 * 0, "vmin_active_protection_b", reg.into());
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
        let reg = self.manufacture_id().read_async().await?;
        callback(46 + 0 * 0, "manufacture_id", reg.into());
        let reg = self.device_id().read_async().await?;
        callback(47 + 0 * 0, "device_id", reg.into());
        Ok(())
    }
    pub fn charge_option_0_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption0A, ::device_driver::RW> {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption0A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption0A::new,
        )
    }
    pub fn charge_option_0_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption0B, ::device_driver::RW> {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption0B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption0B::new,
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
    pub fn charger_status_0_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargerStatus0A, ::device_driver::RW> {
        let address = self.base_address + 32;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargerStatus0A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargerStatus0A::new,
        )
    }
    pub fn charger_status_0_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargerStatus0B, ::device_driver::RO> {
        let address = self.base_address + 33;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargerStatus0B, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ChargerStatus0B::new,
        )
    }
    pub fn prochot_status_reg_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotStatusRegA, ::device_driver::RW> {
        let address = self.base_address + 34;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotStatusRegA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotStatusRegA::new,
        )
    }
    pub fn prochot_status_reg_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotStatusRegB, ::device_driver::RW> {
        let address = self.base_address + 35;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotStatusRegB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotStatusRegB::new,
        )
    }
    pub fn iin_dpm(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::IinDpm, ::device_driver::RW> {
        let address = self.base_address + 36;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::IinDpm, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::IinDpm::new,
        )
    }
    pub fn adc_psys(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcPsys, ::device_driver::RO> {
        let address = self.base_address + 38;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcPsys, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcPsys::new,
        )
    }
    pub fn adc_vbus(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcVbus, ::device_driver::RO> {
        let address = self.base_address + 39;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcVbus, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcVbus::new,
        )
    }
    pub fn adc_ibat_dchrg(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcIbatDchrg, ::device_driver::RO> {
        let address = self.base_address + 40;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcIbatDchrg, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcIbatDchrg::new,
        )
    }
    pub fn adc_ibat_chrg(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcIbatChrg, ::device_driver::RO> {
        let address = self.base_address + 41;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcIbatChrg, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcIbatChrg::new,
        )
    }
    pub fn adc_cmpin(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcCmpin, ::device_driver::RO> {
        let address = self.base_address + 42;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcCmpin, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcCmpin::new,
        )
    }
    pub fn adc_vbat(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcVbat, ::device_driver::RO> {
        let address = self.base_address + 44;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcVbat, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcVbat::new,
        )
    }
    pub fn adc_vsys(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcVsys, ::device_driver::RO> {
        let address = self.base_address + 45;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcVsys, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AdcVsys::new,
        )
    }
    pub fn charge_option_1_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption1A, ::device_driver::RW> {
        let address = self.base_address + 48;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption1A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption1A::new,
        )
    }
    pub fn charge_option_1_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption1B, ::device_driver::RW> {
        let address = self.base_address + 49;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption1B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption1B::new,
        )
    }
    pub fn charge_option_2_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption2A, ::device_driver::RW> {
        let address = self.base_address + 50;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption2A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption2A::new,
        )
    }
    pub fn charge_option_2_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption2B, ::device_driver::RW> {
        let address = self.base_address + 51;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption2B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption2B::new,
        )
    }
    pub fn charge_option_3_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption3A, ::device_driver::RW> {
        let address = self.base_address + 52;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption3A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption3A::new,
        )
    }
    pub fn charge_option_3_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption3B, ::device_driver::RW> {
        let address = self.base_address + 53;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption3B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption3B::new,
        )
    }
    pub fn prochot_option_0_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotOption0A, ::device_driver::RW> {
        let address = self.base_address + 54;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotOption0A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotOption0A::new,
        )
    }
    pub fn prochot_option_0_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotOption0B, ::device_driver::RW> {
        let address = self.base_address + 55;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotOption0B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotOption0B::new,
        )
    }
    pub fn prochot_option_1_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotOption1A, ::device_driver::RW> {
        let address = self.base_address + 56;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotOption1A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotOption1A::new,
        )
    }
    pub fn prochot_option_1_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ProchotOption1B, ::device_driver::RW> {
        let address = self.base_address + 57;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ProchotOption1B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ProchotOption1B::new,
        )
    }
    pub fn adc_option_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcOptionA, ::device_driver::RW> {
        let address = self.base_address + 58;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcOptionA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AdcOptionA::new,
        )
    }
    pub fn adc_option_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AdcOptionB, ::device_driver::RW> {
        let address = self.base_address + 59;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AdcOptionB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AdcOptionB::new,
        )
    }
    pub fn charge_option_4_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption4A, ::device_driver::RW> {
        let address = self.base_address + 60;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption4A, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption4A::new,
        )
    }
    pub fn charge_option_4_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargeOption4B, ::device_driver::RW> {
        let address = self.base_address + 61;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargeOption4B, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ChargeOption4B::new,
        )
    }
    pub fn vmin_active_protection_a(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::VminActiveProtectionA, ::device_driver::RW> {
        let address = self.base_address + 62;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::VminActiveProtectionA, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::VminActiveProtectionA::new,
        )
    }
    pub fn vmin_active_protection_b(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::VminActiveProtectionB, ::device_driver::RW> {
        let address = self.base_address + 63;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::VminActiveProtectionB, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::VminActiveProtectionB::new,
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
    pub fn manufacture_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ManufactureId, ::device_driver::RW> {
        let address = self.base_address + 46;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ManufactureId, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ManufactureId::new,
        )
    }
    pub fn device_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::DeviceId, ::device_driver::RW> {
        let address = self.base_address + 47;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::DeviceId, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::DeviceId::new,
        )
    }
}
/// Module containing the generated fieldsets of the registers and commands
pub mod field_sets {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption0A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption0A {
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
    impl ChargeOption0A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [14] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `chrg_inhibit` field of the register.
        ///
        /// Charge Inhibit. When bit is 0 battery charging will start with valid values in the CHARGE_VOLTAGE() and CHARGE_CURRENT().
        pub fn chrg_inhibit(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_iin_dpm` field of the register.
        ///
        /// IIN_DPM Enable. Host writes this bit to enable IIN_DPM regulation loop. When the IIN_DPM is disabled by the charger (refer to IIN_DPM_AUTO_DISABLE), this bit goes LOW. Under OTG mode, this bit is also used to enable/disable IOTG regulation.
        pub fn en_iin_dpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `en_ldo` field of the register.
        ///
        /// LDO Mode Enable. When battery voltage is below VSYS_MIN(), the charger is in pre-charge with LDO mode enabled.
        pub fn en_ldo(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `ibat_gain` field of the register.
        ///
        /// IBAT Amplifier Ratio. The ratio of voltage on IBAT and voltage across SRP and SRN.
        pub fn ibat_gain(&self) -> super::IbatGain {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `iadpt_gain` field of the register.
        ///
        /// IADPT Amplifier Ratio. The ratio of voltage on IADPT and voltage across ACP and ACN.
        pub fn iadpt_gain(&self) -> super::IadptGain {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_learn` field of the register.
        ///
        /// LEARN mode function enable.
        pub fn en_learn(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `vsys_uvp_enz` field of the register.
        ///
        /// Disable system under voltage protection.
        pub fn vsys_uvp_enz(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_cmp_latch` field of the register.
        ///
        /// Enable Latch of Independent Comparator. Comparator output with effective low. If enabled in PROCHOT profile PP_CMP=1b, STAT_COMP bit keep 1b after triggered until read by host and clear. host can clear CMPOUT pin by toggling this EN_CMP_LATCH bit.
        pub fn en_cmp_latch(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `chrg_inhibit` field of the register.
        ///
        /// Charge Inhibit. When bit is 0 battery charging will start with valid values in the CHARGE_VOLTAGE() and CHARGE_CURRENT().
        pub fn set_chrg_inhibit(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_iin_dpm` field of the register.
        ///
        /// IIN_DPM Enable. Host writes this bit to enable IIN_DPM regulation loop. When the IIN_DPM is disabled by the charger (refer to IIN_DPM_AUTO_DISABLE), this bit goes LOW. Under OTG mode, this bit is also used to enable/disable IOTG regulation.
        pub fn set_en_iin_dpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_ldo` field of the register.
        ///
        /// LDO Mode Enable. When battery voltage is below VSYS_MIN(), the charger is in pre-charge with LDO mode enabled.
        pub fn set_en_ldo(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `ibat_gain` field of the register.
        ///
        /// IBAT Amplifier Ratio. The ratio of voltage on IBAT and voltage across SRP and SRN.
        pub fn set_ibat_gain(&mut self, value: super::IbatGain) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `iadpt_gain` field of the register.
        ///
        /// IADPT Amplifier Ratio. The ratio of voltage on IADPT and voltage across ACP and ACN.
        pub fn set_iadpt_gain(&mut self, value: super::IadptGain) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `en_learn` field of the register.
        ///
        /// LEARN mode function enable.
        pub fn set_en_learn(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `vsys_uvp_enz` field of the register.
        ///
        /// Disable system under voltage protection.
        pub fn set_vsys_uvp_enz(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_cmp_latch` field of the register.
        ///
        /// Enable Latch of Independent Comparator. Comparator output with effective low. If enabled in PROCHOT profile PP_CMP=1b, STAT_COMP bit keep 1b after triggered until read by host and clear. host can clear CMPOUT pin by toggling this EN_CMP_LATCH bit.
        pub fn set_en_cmp_latch(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption0A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption0A> for [u8; 1] {
        fn from(val: ChargeOption0A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption0A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption0A");
            d.field("chrg_inhibit", &self.chrg_inhibit());
            d.field("en_iin_dpm", &self.en_iin_dpm());
            d.field("en_ldo", &self.en_ldo());
            d.field("ibat_gain", &self.ibat_gain());
            d.field("iadpt_gain", &self.iadpt_gain());
            d.field("en_learn", &self.en_learn());
            d.field("vsys_uvp_enz", &self.vsys_uvp_enz());
            d.field("en_cmp_latch", &self.en_cmp_latch());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption0A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption0A {{ ");
            defmt::write!(f, "chrg_inhibit: {=bool}, ", &self.chrg_inhibit());
            defmt::write!(f, "en_iin_dpm: {=bool}, ", &self.en_iin_dpm());
            defmt::write!(f, "en_ldo: {=bool}, ", &self.en_ldo());
            defmt::write!(f, "ibat_gain: {}, ", &self.ibat_gain());
            defmt::write!(f, "iadpt_gain: {}, ", &self.iadpt_gain());
            defmt::write!(f, "en_learn: {=bool}, ", &self.en_learn());
            defmt::write!(f, "vsys_uvp_enz: {=bool}, ", &self.vsys_uvp_enz());
            defmt::write!(f, "en_cmp_latch: {=bool}, ", &self.en_cmp_latch());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption0A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption0A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption0A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption0A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption0A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption0A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption0A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption0B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption0B {
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
    impl ChargeOption0B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [231] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `dis_strgrv` field of the register.
        ///
        /// Switching HS MOSFET turn on gate drive strength.
        pub fn dis_strgrv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `pwm_freq` field of the register.
        ///
        /// Switching Frequency Selection.
        pub fn pwm_freq(&self) -> super::SwitchingFreq {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_ooa` field of the register.
        ///
        /// Out-of-Audio Enable.
        pub fn en_ooa(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `otg_on_chrgok` field of the register.
        ///
        /// Add OTG to CHRG_OK.
        pub fn otg_on_chrgok(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `iin_dpm_auto_disable` field of the register.
        ///
        /// IIN_DPM Auto Disable.
        pub fn iin_dpm_auto_disable(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `wdtmr_adj` field of the register.
        ///
        /// WATCHDOG Timer Adjust. Set maximum delay between consecutive EC host write of charge voltage or charge current command.
        pub fn wdtmr_adj(&self) -> super::MaxDelay {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 7) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_lwpwr` field of the register.
        ///
        /// Low Power Mode enable.
        pub fn en_lwpwr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `dis_strgrv` field of the register.
        ///
        /// Switching HS MOSFET turn on gate drive strength.
        pub fn set_dis_strgrv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `pwm_freq` field of the register.
        ///
        /// Switching Frequency Selection.
        pub fn set_pwm_freq(&mut self, value: super::SwitchingFreq) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_ooa` field of the register.
        ///
        /// Out-of-Audio Enable.
        pub fn set_en_ooa(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `otg_on_chrgok` field of the register.
        ///
        /// Add OTG to CHRG_OK.
        pub fn set_otg_on_chrgok(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `iin_dpm_auto_disable` field of the register.
        ///
        /// IIN_DPM Auto Disable.
        pub fn set_iin_dpm_auto_disable(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `wdtmr_adj` field of the register.
        ///
        /// WATCHDOG Timer Adjust. Set maximum delay between consecutive EC host write of charge voltage or charge current command.
        pub fn set_wdtmr_adj(&mut self, value: super::MaxDelay) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 7, &mut self.bits) };
        }
        ///Write the `en_lwpwr` field of the register.
        ///
        /// Low Power Mode enable.
        pub fn set_en_lwpwr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption0B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption0B> for [u8; 1] {
        fn from(val: ChargeOption0B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption0B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption0B");
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
    impl defmt::Format for ChargeOption0B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption0B {{ ");
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
    impl core::ops::BitAnd for ChargeOption0B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption0B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption0B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption0B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption0B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption0B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption0B {
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
        /// Charge current setting
        pub fn charge_current(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 13) };
            raw
        }
        ///Write the `charge_current` field of the register.
        ///
        /// Charge current setting
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
        /// Charge voltage setting.
        pub fn charge_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 3, 15) };
            raw
        }
        ///Write the `charge_voltage` field of the register.
        ///
        /// Charge voltage setting.
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
    pub struct ChargerStatus0A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargerStatus0A {
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
    impl ChargerStatus0A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `fault_otg_uvp` field of the register.
        ///
        /// The faults are latched until a read from host.
        pub fn fault_otg_uvp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `fault_otg_ovp` field of the register.
        ///
        /// The faults are latched until a read from host.
        pub fn fault_otg_ovp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `fault_force_converter_off` field of the register.
        ///
        /// The faults are latched until a read from host.
        pub fn fault_force_converter_off(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `fault_vsys_uvp` field of the register.
        ///
        /// VSYS_UVP fault status and clear. VSYS_UVP fault is latched until a clear from host by writing this bit to 0.
        pub fn fault_vsys_uvp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `fault_sysovp` field of the register.
        ///
        /// SYSOVP Status and Clear When the SYSOVP occurs, this bit is HIGH. During the SYSOVP, the converter is disabled. After the SYSOVP is removed, the user must write a 0 to this bit or unplug the adapter to clear the SYSOVP condition to enable the converter again.
        pub fn fault_sysovp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `fault_acoc` field of the register.
        ///
        /// The faults are latched until a read from host.
        pub fn fault_acoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `fault_batoc` field of the register.
        ///
        /// The faults are latched until a read from host.
        pub fn fault_batoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `fault_acov` field of the register.
        ///
        /// The faults are latched until a read from host.
        pub fn fault_acov(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `fault_vsys_uvp` field of the register.
        ///
        /// VSYS_UVP fault status and clear. VSYS_UVP fault is latched until a clear from host by writing this bit to 0.
        pub fn set_fault_vsys_uvp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `fault_sysovp` field of the register.
        ///
        /// SYSOVP Status and Clear When the SYSOVP occurs, this bit is HIGH. During the SYSOVP, the converter is disabled. After the SYSOVP is removed, the user must write a 0 to this bit or unplug the adapter to clear the SYSOVP condition to enable the converter again.
        pub fn set_fault_sysovp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargerStatus0A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargerStatus0A> for [u8; 1] {
        fn from(val: ChargerStatus0A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargerStatus0A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargerStatus0A");
            d.field("fault_otg_uvp", &self.fault_otg_uvp());
            d.field("fault_otg_ovp", &self.fault_otg_ovp());
            d.field("fault_force_converter_off", &self.fault_force_converter_off());
            d.field("fault_vsys_uvp", &self.fault_vsys_uvp());
            d.field("fault_sysovp", &self.fault_sysovp());
            d.field("fault_acoc", &self.fault_acoc());
            d.field("fault_batoc", &self.fault_batoc());
            d.field("fault_acov", &self.fault_acov());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargerStatus0A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargerStatus0A {{ ");
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
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargerStatus0A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargerStatus0A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargerStatus0A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargerStatus0A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargerStatus0A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargerStatus0A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargerStatus0A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargerStatus0B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargerStatus0B {
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
    impl ChargerStatus0B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `in_otg` field of the register.
        ///
        /// OTG?
        pub fn in_otg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `in_pchrg` field of the register.
        ///
        /// Pre charge?
        pub fn in_pchrg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `in_fchrg` field of the register.
        ///
        /// Fast charge?
        pub fn in_fchrg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `in_iin_dpm` field of the register.
        ///
        /// The fault is latched until a clear from host by writing this bit to 0.
        pub fn in_iin_dpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `in_vind_pm` field of the register.
        ///
        /// VINDPM during forward mode, or voltage regulation during OTG mode?
        pub fn in_vind_pm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `in_vap` field of the register.
        ///
        /// VAP Mode?
        pub fn in_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `ico_done` field of the register.
        ///
        /// After the ICO routine is successfully executed, the bit goes 1.
        pub fn ico_done(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `stat_ac` field of the register.
        ///
        /// Input source status. STAT_AC is valid as long as VBUS go within 3.5-V to 26-V range. It is different from CHRG_OK bit, When CHRG_OK is valid, STAT_AC must be valid, but if STAT_AC is valid, it is not necessary CHRG_OK is valid. There are Force converter off, ACOC, TSHUT , SYSOVP, VSYS_UVP, BATOVP can pull low CHRG_OK.
        pub fn stat_ac(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `in_iin_dpm` field of the register.
        ///
        /// The fault is latched until a clear from host by writing this bit to 0.
        pub fn set_in_iin_dpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `in_vind_pm` field of the register.
        ///
        /// VINDPM during forward mode, or voltage regulation during OTG mode?
        pub fn set_in_vind_pm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargerStatus0B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargerStatus0B> for [u8; 1] {
        fn from(val: ChargerStatus0B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargerStatus0B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargerStatus0B");
            d.field("in_otg", &self.in_otg());
            d.field("in_pchrg", &self.in_pchrg());
            d.field("in_fchrg", &self.in_fchrg());
            d.field("in_iin_dpm", &self.in_iin_dpm());
            d.field("in_vind_pm", &self.in_vind_pm());
            d.field("in_vap", &self.in_vap());
            d.field("ico_done", &self.ico_done());
            d.field("stat_ac", &self.stat_ac());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargerStatus0B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargerStatus0B {{ ");
            defmt::write!(f, "in_otg: {=bool}, ", &self.in_otg());
            defmt::write!(f, "in_pchrg: {=bool}, ", &self.in_pchrg());
            defmt::write!(f, "in_fchrg: {=bool}, ", &self.in_fchrg());
            defmt::write!(f, "in_iin_dpm: {=bool}, ", &self.in_iin_dpm());
            defmt::write!(f, "in_vind_pm: {=bool}, ", &self.in_vind_pm());
            defmt::write!(f, "in_vap: {=bool}, ", &self.in_vap());
            defmt::write!(f, "ico_done: {=bool}, ", &self.ico_done());
            defmt::write!(f, "stat_ac: {=bool}, ", &self.stat_ac());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargerStatus0B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargerStatus0B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargerStatus0B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargerStatus0B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargerStatus0B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargerStatus0B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargerStatus0B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotStatusRegA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotStatusRegA {
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
    impl ProchotStatusRegA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `stat_adapter_removal` field of the register.
        ///
        /// Adapter removed?
        pub fn stat_adapter_removal(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `stat_battery_removal` field of the register.
        ///
        /// Battery removed?
        pub fn stat_battery_removal(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `stat_vsys` field of the register.
        ///
        /// VSYS status triggered?
        pub fn stat_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `stat_idchg_1` field of the register.
        ///
        /// IDCHG1 status triggered?
        pub fn stat_idchg_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `stat_inom` field of the register.
        ///
        /// INOM status triggered?
        pub fn stat_inom(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `stat_icrit` field of the register.
        ///
        /// ICRIT status triggered?
        pub fn stat_icrit(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `stat_comp` field of the register.
        ///
        /// COMP status triggered?
        pub fn stat_comp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `stat_vindpm` field of the register.
        ///
        /// VINDPM status triggered?
        pub fn stat_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `stat_vindpm` field of the register.
        ///
        /// VINDPM status triggered?
        pub fn set_stat_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotStatusRegA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotStatusRegA> for [u8; 1] {
        fn from(val: ProchotStatusRegA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotStatusRegA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotStatusRegA");
            d.field("stat_adapter_removal", &self.stat_adapter_removal());
            d.field("stat_battery_removal", &self.stat_battery_removal());
            d.field("stat_vsys", &self.stat_vsys());
            d.field("stat_idchg_1", &self.stat_idchg_1());
            d.field("stat_inom", &self.stat_inom());
            d.field("stat_icrit", &self.stat_icrit());
            d.field("stat_comp", &self.stat_comp());
            d.field("stat_vindpm", &self.stat_vindpm());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotStatusRegA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotStatusRegA {{ ");
            defmt::write!(f, "stat_adapter_removal: {=bool}, ", &self.stat_adapter_removal());
            defmt::write!(f, "stat_battery_removal: {=bool}, ", &self.stat_battery_removal());
            defmt::write!(f, "stat_vsys: {=bool}, ", &self.stat_vsys());
            defmt::write!(f, "stat_idchg_1: {=bool}, ", &self.stat_idchg_1());
            defmt::write!(f, "stat_inom: {=bool}, ", &self.stat_inom());
            defmt::write!(f, "stat_icrit: {=bool}, ", &self.stat_icrit());
            defmt::write!(f, "stat_comp: {=bool}, ", &self.stat_comp());
            defmt::write!(f, "stat_vindpm: {=bool}, ", &self.stat_vindpm());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotStatusRegA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotStatusRegA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotStatusRegA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotStatusRegA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotStatusRegA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotStatusRegA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotStatusRegA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotStatusRegB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotStatusRegB {
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
    impl ProchotStatusRegB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [56] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `stat_exit_vap` field of the register.
        ///
        /// PROCHOT_EXIT_VAP is active?
        pub fn stat_exit_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `stat_vap_fail` field of the register.
        ///
        /// In VAP failure?
        pub fn stat_vap_fail(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `tshut` field of the register.
        ///
        /// TSHUT trigger
        pub fn tshut(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `prochot_clear` field of the register.
        ///
        /// PROCHOT Pulse Clear. Clear PROCHOT pulse when EN_PROCHOT_EXT=0b.
        pub fn prochot_clear(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `prochot_width` field of the register.
        ///
        /// PROCHOT Pulse Width when EN_PROCHOT_EXT = 0b.
        pub fn prochot_width(&self) -> super::ProchotPulseWidth {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_prochot_ext` field of the register.
        ///
        /// PROCHOT Pulse Extension Enable. When pulse extension is enabled, keep the PROCHOT pin voltage LOW until host writes PROCHOT_CLEAR= 0b.
        pub fn en_prochot_ext(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Write the `stat_exit_vap` field of the register.
        ///
        /// PROCHOT_EXIT_VAP is active?
        pub fn set_stat_exit_vap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `stat_vap_fail` field of the register.
        ///
        /// In VAP failure?
        pub fn set_stat_vap_fail(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `prochot_clear` field of the register.
        ///
        /// PROCHOT Pulse Clear. Clear PROCHOT pulse when EN_PROCHOT_EXT=0b.
        pub fn set_prochot_clear(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `prochot_width` field of the register.
        ///
        /// PROCHOT Pulse Width when EN_PROCHOT_EXT = 0b.
        pub fn set_prochot_width(&mut self, value: super::ProchotPulseWidth) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 6, &mut self.bits) };
        }
        ///Write the `en_prochot_ext` field of the register.
        ///
        /// PROCHOT Pulse Extension Enable. When pulse extension is enabled, keep the PROCHOT pin voltage LOW until host writes PROCHOT_CLEAR= 0b.
        pub fn set_en_prochot_ext(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotStatusRegB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotStatusRegB> for [u8; 1] {
        fn from(val: ProchotStatusRegB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotStatusRegB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotStatusRegB");
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
    impl defmt::Format for ProchotStatusRegB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotStatusRegB {{ ");
            defmt::write!(f, "stat_exit_vap: {=bool}, ", &self.stat_exit_vap());
            defmt::write!(f, "stat_vap_fail: {=bool}, ", &self.stat_vap_fail());
            defmt::write!(f, "tshut: {=bool}, ", &self.tshut());
            defmt::write!(f, "prochot_clear: {=bool}, ", &self.prochot_clear());
            defmt::write!(f, "prochot_width: {}, ", &self.prochot_width());
            defmt::write!(f, "en_prochot_ext: {=bool}, ", &self.en_prochot_ext());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotStatusRegB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotStatusRegB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotStatusRegB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotStatusRegB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotStatusRegB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotStatusRegB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotStatusRegB {
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
        /// Actual input current limit
        pub fn iin_dpm(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 15) };
            raw
        }
        ///Write the `iin_dpm` field of the register.
        ///
        /// Actual input current limit
        pub fn set_iin_dpm(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 15, &mut self.bits) };
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
    pub struct AdcPsys {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcPsys {
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
    impl AdcPsys {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `adc_psys` field of the register.
        ///
        /// 8-bit Digital Output of System Power
        pub fn adc_psys(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
    }
    impl From<[u8; 1]> for AdcPsys {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcPsys> for [u8; 1] {
        fn from(val: AdcPsys) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcPsys {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcPsys");
            d.field("adc_psys", &self.adc_psys());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcPsys {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcPsys {{ ");
            defmt::write!(f, "adc_psys: {=u8}, ", &self.adc_psys());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcPsys {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcPsys {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcPsys {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcPsys {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcPsys {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcPsys {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcPsys {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcVbus {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcVbus {
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
    impl AdcVbus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `adc_vbus` field of the register.
        ///
        /// 8-bit Digital Output of Input Voltage
        pub fn adc_vbus(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
    }
    impl From<[u8; 1]> for AdcVbus {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcVbus> for [u8; 1] {
        fn from(val: AdcVbus) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcVbus {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcVbus");
            d.field("adc_vbus", &self.adc_vbus());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcVbus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcVbus {{ ");
            defmt::write!(f, "adc_vbus: {=u8}, ", &self.adc_vbus());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcVbus {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcVbus {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcVbus {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcVbus {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcVbus {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcVbus {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcVbus {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcIbatDchrg {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcIbatDchrg {
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
    impl AdcIbatDchrg {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `adc_ibat_dchrg` field of the register.
        ///
        /// 7-bit Digital Output of Battery Charge Current
        pub fn adc_ibat_dchrg(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 7) };
            raw
        }
    }
    impl From<[u8; 1]> for AdcIbatDchrg {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcIbatDchrg> for [u8; 1] {
        fn from(val: AdcIbatDchrg) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcIbatDchrg {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcIbatDchrg");
            d.field("adc_ibat_dchrg", &self.adc_ibat_dchrg());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcIbatDchrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcIbatDchrg {{ ");
            defmt::write!(f, "adc_ibat_dchrg: {=u8}, ", &self.adc_ibat_dchrg());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcIbatDchrg {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcIbatDchrg {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcIbatDchrg {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcIbatDchrg {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcIbatDchrg {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcIbatDchrg {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcIbatDchrg {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcIbatChrg {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcIbatChrg {
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
    impl AdcIbatChrg {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `adc_ibat_chrg` field of the register.
        ///
        /// 7-bit Digital Output of Battery Discharge Current
        pub fn adc_ibat_chrg(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 7) };
            raw
        }
    }
    impl From<[u8; 1]> for AdcIbatChrg {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcIbatChrg> for [u8; 1] {
        fn from(val: AdcIbatChrg) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcIbatChrg {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcIbatChrg");
            d.field("adc_ibat_chrg", &self.adc_ibat_chrg());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcIbatChrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcIbatChrg {{ ");
            defmt::write!(f, "adc_ibat_chrg: {=u8}, ", &self.adc_ibat_chrg());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcIbatChrg {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcIbatChrg {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcIbatChrg {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcIbatChrg {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcIbatChrg {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcIbatChrg {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcIbatChrg {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcCmpin {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcCmpin {
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
    impl AdcCmpin {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `adc_cmpin` field of the register.
        ///
        /// 8-bit Digital Output of CMPIN voltage
        pub fn adc_cmpin(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
    }
    impl From<[u8; 1]> for AdcCmpin {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcCmpin> for [u8; 1] {
        fn from(val: AdcCmpin) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcCmpin {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcCmpin");
            d.field("adc_cmpin", &self.adc_cmpin());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcCmpin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcCmpin {{ ");
            defmt::write!(f, "adc_cmpin: {=u8}, ", &self.adc_cmpin());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcCmpin {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcCmpin {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcCmpin {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcCmpin {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcCmpin {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcCmpin {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcCmpin {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcVbat {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcVbat {
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
    impl AdcVbat {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `adc_vbat` field of the register.
        ///
        /// 8-bit Digital Output of Battery Voltage
        pub fn adc_vbat(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
    }
    impl From<[u8; 1]> for AdcVbat {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcVbat> for [u8; 1] {
        fn from(val: AdcVbat) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcVbat {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcVbat");
            d.field("adc_vbat", &self.adc_vbat());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcVbat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcVbat {{ ");
            defmt::write!(f, "adc_vbat: {=u8}, ", &self.adc_vbat());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcVbat {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcVbat {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcVbat {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcVbat {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcVbat {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcVbat {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcVbat {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcVsys {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcVsys {
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
    impl AdcVsys {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `adc_vsys` field of the register.
        ///
        /// 8-bit Digital Output of System Voltage
        pub fn adc_vsys(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
    }
    impl From<[u8; 1]> for AdcVsys {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcVsys> for [u8; 1] {
        fn from(val: AdcVsys) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcVsys {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcVsys");
            d.field("adc_vsys", &self.adc_vsys());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcVsys {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcVsys {{ ");
            defmt::write!(f, "adc_vsys: {=u8}, ", &self.adc_vsys());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcVsys {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcVsys {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcVsys {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcVsys {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcVsys {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcVsys {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcVsys {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption1A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption1A {
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
    impl ChargeOption1A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `auto_wakeup_en` field of the register.
        ///
        /// Auto Wakeup Enable When this bit is HIGH, if the battery is below minimum system voltage (REG0x0D/0C()), the device will automatically enable 128 mA charging current for 30 mins. When the battery is charged up above minimum system voltage, charge will terminate and the bit is reset to LOW
        pub fn auto_wakeup_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_ship_dchg` field of the register.
        ///
        /// Discharge SRN for Shipping Mode. Used to discharge SRN pin capacitor voltage which is necessary for battery gauge device shipping mode.
        pub fn en_ship_dchg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `en_ptm` field of the register.
        ///
        /// PTM enable register bit, it will automatically reset to zero.
        pub fn en_ptm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `force_conv_off` field of the register.
        ///
        /// Force Converter Off function When independent comparator triggers, (CMPOUT pin pulled down) converter latches off, at the same time, CHRG_OK signal goes LOW to notify the system. Charge current is also set to zero internally, but charge current register setting keeps the same. To get out of converter latches off, firstly the CMPOUT should be released to high and secondly FORCE_CONV_OFF bit should be cleared (=0b).
        pub fn force_conv_off(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `cmp_deg` field of the register.
        ///
        /// Independent comparator deglitch time, only applied to the falling edge of CMPOUT (HIGH to LOW).
        pub fn cmp_deg(&self) -> super::ComparatorDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `cmp_pol` field of the register.
        ///
        /// Independent Comparator output Polarity
        pub fn cmp_pol(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `cmp_ref` field of the register.
        ///
        /// Independent Comparator Internal Reference. The datasheet indicates 0 = 2.3V reference 1 = 1.2V reference
        pub fn cmp_ref(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `auto_wakeup_en` field of the register.
        ///
        /// Auto Wakeup Enable When this bit is HIGH, if the battery is below minimum system voltage (REG0x0D/0C()), the device will automatically enable 128 mA charging current for 30 mins. When the battery is charged up above minimum system voltage, charge will terminate and the bit is reset to LOW
        pub fn set_auto_wakeup_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_ship_dchg` field of the register.
        ///
        /// Discharge SRN for Shipping Mode. Used to discharge SRN pin capacitor voltage which is necessary for battery gauge device shipping mode.
        pub fn set_en_ship_dchg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_ptm` field of the register.
        ///
        /// PTM enable register bit, it will automatically reset to zero.
        pub fn set_en_ptm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `force_conv_off` field of the register.
        ///
        /// Force Converter Off function When independent comparator triggers, (CMPOUT pin pulled down) converter latches off, at the same time, CHRG_OK signal goes LOW to notify the system. Charge current is also set to zero internally, but charge current register setting keeps the same. To get out of converter latches off, firstly the CMPOUT should be released to high and secondly FORCE_CONV_OFF bit should be cleared (=0b).
        pub fn set_force_conv_off(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `cmp_deg` field of the register.
        ///
        /// Independent comparator deglitch time, only applied to the falling edge of CMPOUT (HIGH to LOW).
        pub fn set_cmp_deg(&mut self, value: super::ComparatorDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 6, &mut self.bits) };
        }
        ///Write the `cmp_pol` field of the register.
        ///
        /// Independent Comparator output Polarity
        pub fn set_cmp_pol(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `cmp_ref` field of the register.
        ///
        /// Independent Comparator Internal Reference. The datasheet indicates 0 = 2.3V reference 1 = 1.2V reference
        pub fn set_cmp_ref(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption1A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption1A> for [u8; 1] {
        fn from(val: ChargeOption1A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption1A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption1A");
            d.field("auto_wakeup_en", &self.auto_wakeup_en());
            d.field("en_ship_dchg", &self.en_ship_dchg());
            d.field("en_ptm", &self.en_ptm());
            d.field("force_conv_off", &self.force_conv_off());
            d.field("cmp_deg", &self.cmp_deg());
            d.field("cmp_pol", &self.cmp_pol());
            d.field("cmp_ref", &self.cmp_ref());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption1A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption1A {{ ");
            defmt::write!(f, "auto_wakeup_en: {=bool}, ", &self.auto_wakeup_en());
            defmt::write!(f, "en_ship_dchg: {=bool}, ", &self.en_ship_dchg());
            defmt::write!(f, "en_ptm: {=bool}, ", &self.en_ptm());
            defmt::write!(f, "force_conv_off: {=bool}, ", &self.force_conv_off());
            defmt::write!(f, "cmp_deg: {}, ", &self.cmp_deg());
            defmt::write!(f, "cmp_pol: {=bool}, ", &self.cmp_pol());
            defmt::write!(f, "cmp_ref: {=bool}, ", &self.cmp_ref());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption1A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption1A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption1A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption1A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption1A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption1A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption1A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption1B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption1B {
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
    impl ChargeOption1B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [51] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `en_fast_5_mohm` field of the register.
        ///
        /// Enable fast compensation to increase bandwidth under 5 mΩ RAC (RSNS_RAC=1b) for input current up to 6.4-A application (the fast compensation will only work when IADPT pin is configured less than 160 kΩ
        pub fn en_fast_5_mohm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `psys_ratio` field of the register.
        ///
        /// PSYS Gain. Ratio of PSYS output current vs total input and battery power.
        pub fn psys_ratio(&self) -> super::PsysGain {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `rsns_rsr` field of the register.
        ///
        /// Charge sense resistor RSR.
        pub fn rsns_rsr(&self) -> super::ChargeSenseResistorRsr {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `rsns_rac` field of the register.
        ///
        /// Input sense resistor RAC.
        pub fn rsns_rac(&self) -> super::InputSenseResistorRac {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `psys_config` field of the register.
        ///
        /// PSYS Enable and Definition Register Enable PSYS sensing circuit and output buffer (whole PSYS circuit). In low power mode (EN_LWPWR=1b), PSYS sensing and buffer are always disabled regardless of this bit value.
        pub fn psys_config(&self) -> super::PsysEnable {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_prochot_lpwr` field of the register.
        ///
        /// Enable PROCHOT during battery only low power mode With battery only, enable VSYS in PROCHOT with low power consumption. Do not enable this function with adapter present.
        pub fn en_prochot_lpwr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_ibat` field of the register.
        ///
        /// IBAT Enable. In low power mode (EN_LWPWR=1b), IBAT buffer is always disabled regardless of this bit value.
        pub fn en_ibat(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `en_fast_5_mohm` field of the register.
        ///
        /// Enable fast compensation to increase bandwidth under 5 mΩ RAC (RSNS_RAC=1b) for input current up to 6.4-A application (the fast compensation will only work when IADPT pin is configured less than 160 kΩ
        pub fn set_en_fast_5_mohm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `psys_ratio` field of the register.
        ///
        /// PSYS Gain. Ratio of PSYS output current vs total input and battery power.
        pub fn set_psys_ratio(&mut self, value: super::PsysGain) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `rsns_rsr` field of the register.
        ///
        /// Charge sense resistor RSR.
        pub fn set_rsns_rsr(&mut self, value: super::ChargeSenseResistorRsr) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `rsns_rac` field of the register.
        ///
        /// Input sense resistor RAC.
        pub fn set_rsns_rac(&mut self, value: super::InputSenseResistorRac) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `psys_config` field of the register.
        ///
        /// PSYS Enable and Definition Register Enable PSYS sensing circuit and output buffer (whole PSYS circuit). In low power mode (EN_LWPWR=1b), PSYS sensing and buffer are always disabled regardless of this bit value.
        pub fn set_psys_config(&mut self, value: super::PsysEnable) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 6, &mut self.bits) };
        }
        ///Write the `en_prochot_lpwr` field of the register.
        ///
        /// Enable PROCHOT during battery only low power mode With battery only, enable VSYS in PROCHOT with low power consumption. Do not enable this function with adapter present.
        pub fn set_en_prochot_lpwr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_ibat` field of the register.
        ///
        /// IBAT Enable. In low power mode (EN_LWPWR=1b), IBAT buffer is always disabled regardless of this bit value.
        pub fn set_en_ibat(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption1B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption1B> for [u8; 1] {
        fn from(val: ChargeOption1B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption1B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption1B");
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
    impl defmt::Format for ChargeOption1B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption1B {{ ");
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
    impl core::ops::BitAnd for ChargeOption1B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption1B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption1B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption1B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption1B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption1B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption1B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption2A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption2A {
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
    impl ChargeOption2A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [183] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `batoc_vth` field of the register.
        ///
        /// Set battery discharge overcurrent threshold as percentage of PROCHOT battery discharge current limit.
        pub fn batoc_vth(&self) -> super::BatocVth {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_batdoc` field of the register.
        ///
        /// Battery discharge overcurrent (BATDOC) protection enable.
        pub fn en_batdoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `acoc_vth` field of the register.
        ///
        /// ACOC Limit. Set ACOC threshold as percentage of ILIM2_VTH with current sensed from RAC.
        pub fn acoc_vth(&self) -> super::AcocLimit {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_acoc` field of the register.
        ///
        /// Input overcurrent (ACOC) protection enable.
        pub fn en_acoc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `acx_ocp` field of the register.
        ///
        /// Input current OCP threshold by sensing ACP-ACN.
        pub fn acx_ocp(&self) -> super::AcxOcp {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `q_2_ocp` field of the register.
        ///
        /// Q2 OCP threshold by sensing Q2 VDS
        pub fn q_2_ocp(&self) -> super::Q2Ocp {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_ichg_idchg` field of the register.
        ///
        /// IBAT pin monitor selection for discharge current and charge current.
        pub fn en_ichg_idchg(&self) -> super::IBatPinSelect {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `en_extilim` field of the register.
        ///
        /// Enable ILIM_HIZ pin to set input current limit.
        pub fn en_extilim(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `batoc_vth` field of the register.
        ///
        /// Set battery discharge overcurrent threshold as percentage of PROCHOT battery discharge current limit.
        pub fn set_batoc_vth(&mut self, value: super::BatocVth) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_batdoc` field of the register.
        ///
        /// Battery discharge overcurrent (BATDOC) protection enable.
        pub fn set_en_batdoc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `acoc_vth` field of the register.
        ///
        /// ACOC Limit. Set ACOC threshold as percentage of ILIM2_VTH with current sensed from RAC.
        pub fn set_acoc_vth(&mut self, value: super::AcocLimit) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `en_acoc` field of the register.
        ///
        /// Input overcurrent (ACOC) protection enable.
        pub fn set_en_acoc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `acx_ocp` field of the register.
        ///
        /// Input current OCP threshold by sensing ACP-ACN.
        pub fn set_acx_ocp(&mut self, value: super::AcxOcp) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `q_2_ocp` field of the register.
        ///
        /// Q2 OCP threshold by sensing Q2 VDS
        pub fn set_q_2_ocp(&mut self, value: super::Q2Ocp) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `en_ichg_idchg` field of the register.
        ///
        /// IBAT pin monitor selection for discharge current and charge current.
        pub fn set_en_ichg_idchg(&mut self, value: super::IBatPinSelect) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_extilim` field of the register.
        ///
        /// Enable ILIM_HIZ pin to set input current limit.
        pub fn set_en_extilim(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption2A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption2A> for [u8; 1] {
        fn from(val: ChargeOption2A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption2A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption2A");
            d.field("batoc_vth", &self.batoc_vth());
            d.field("en_batdoc", &self.en_batdoc());
            d.field("acoc_vth", &self.acoc_vth());
            d.field("en_acoc", &self.en_acoc());
            d.field("acx_ocp", &self.acx_ocp());
            d.field("q_2_ocp", &self.q_2_ocp());
            d.field("en_ichg_idchg", &self.en_ichg_idchg());
            d.field("en_extilim", &self.en_extilim());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption2A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption2A {{ ");
            defmt::write!(f, "batoc_vth: {}, ", &self.batoc_vth());
            defmt::write!(f, "en_batdoc: {=bool}, ", &self.en_batdoc());
            defmt::write!(f, "acoc_vth: {}, ", &self.acoc_vth());
            defmt::write!(f, "en_acoc: {=bool}, ", &self.en_acoc());
            defmt::write!(f, "acx_ocp: {}, ", &self.acx_ocp());
            defmt::write!(f, "q_2_ocp: {}, ", &self.q_2_ocp());
            defmt::write!(f, "en_ichg_idchg: {}, ", &self.en_ichg_idchg());
            defmt::write!(f, "en_extilim: {=bool}, ", &self.en_extilim());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption2A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption2A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption2A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption2A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption2A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption2A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption2A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption2B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption2B {
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
    impl ChargeOption2B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `pkpwr_tmax` field of the register.
        ///
        /// Peak power mode overload and relax cycle time.
        pub fn pkpwr_tmax(&self) -> super::PkpwrTmax {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `pkpwr_relax_stat` field of the register.
        ///
        /// Indicator that the device is in relaxation cycle
        pub fn pkpwr_relax_stat(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `stat_pkpwr_ovld` field of the register.
        ///
        /// Indicator that the device is in overloading cycle
        pub fn stat_pkpwr_ovld(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `en_pkpwr_vsys` field of the register.
        ///
        /// Enable Peak Power Mode triggered by system voltage under-shoot.
        pub fn en_pkpwr_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `en_pkpwr_iin_dpm` field of the register.
        ///
        /// Enable Peak Power Mode triggered by input current overshoot.
        pub fn en_pkpwr_iin_dpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `pkpwr_tovld_deg` field of the register.
        ///
        /// Input Overload time in Peak Power Mode.
        pub fn pkpwr_tovld_deg(&self) -> super::PkpwrTovldDeg {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `pkpwr_tmax` field of the register.
        ///
        /// Peak power mode overload and relax cycle time.
        pub fn set_pkpwr_tmax(&mut self, value: super::PkpwrTmax) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 2, &mut self.bits) };
        }
        ///Write the `pkpwr_relax_stat` field of the register.
        ///
        /// Indicator that the device is in relaxation cycle
        pub fn set_pkpwr_relax_stat(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `stat_pkpwr_ovld` field of the register.
        ///
        /// Indicator that the device is in overloading cycle
        pub fn set_stat_pkpwr_ovld(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `en_pkpwr_vsys` field of the register.
        ///
        /// Enable Peak Power Mode triggered by system voltage under-shoot.
        pub fn set_en_pkpwr_vsys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `en_pkpwr_iin_dpm` field of the register.
        ///
        /// Enable Peak Power Mode triggered by input current overshoot.
        pub fn set_en_pkpwr_iin_dpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `pkpwr_tovld_deg` field of the register.
        ///
        /// Input Overload time in Peak Power Mode.
        pub fn set_pkpwr_tovld_deg(&mut self, value: super::PkpwrTovldDeg) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption2B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption2B> for [u8; 1] {
        fn from(val: ChargeOption2B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption2B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption2B");
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
    impl defmt::Format for ChargeOption2B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption2B {{ ");
            defmt::write!(f, "pkpwr_tmax: {}, ", &self.pkpwr_tmax());
            defmt::write!(f, "pkpwr_relax_stat: {=bool}, ", &self.pkpwr_relax_stat());
            defmt::write!(f, "stat_pkpwr_ovld: {=bool}, ", &self.stat_pkpwr_ovld());
            defmt::write!(f, "en_pkpwr_vsys: {=bool}, ", &self.en_pkpwr_vsys());
            defmt::write!(f, "en_pkpwr_iin_dpm: {=bool}, ", &self.en_pkpwr_iin_dpm());
            defmt::write!(f, "pkpwr_tovld_deg: {}, ", &self.pkpwr_tovld_deg());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption2B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption2B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption2B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption2B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption2B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption2B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption2B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption3A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption3A {
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
    impl ChargeOption3A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [52] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `psys_otg_idchg` field of the register.
        ///
        /// PSYS definition during OTG mode.
        pub fn psys_otg_idchg(&self) -> super::PsysOtg {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `batfetoff_hiz` field of the register.
        ///
        /// BATFET off during HIZ mode?
        pub fn batfetoff_hiz(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `cmp_en` field of the register.
        ///
        /// Enable Independent Comparator with effective low
        pub fn cmp_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `il_avg` field of the register.
        ///
        /// Inductor average current clamp.
        pub fn il_avg(&self) -> super::IlAvgClamp {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `otg_vap_mode` field of the register.
        ///
        /// The selection of the external OTG/VAP/FRS pin control. Don't recommend to change pin control after OTG/VAP/FRS pin is pulled high.
        pub fn otg_vap_mode(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `en_vbus_vap` field of the register.
        ///
        /// Enable the VBUS VAP for VAP operation mode 2&3
        pub fn en_vbus_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `batfet_enz` field of the register.
        ///
        /// Turn off BATFET under battery only mode. If charger is not in battery only mode this bit is not allowed to be written to 1. Under battery only OTG mode, this bit is forced to be 0b.
        pub fn batfet_enz(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `psys_otg_idchg` field of the register.
        ///
        /// PSYS definition during OTG mode.
        pub fn set_psys_otg_idchg(&mut self, value: super::PsysOtg) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `batfetoff_hiz` field of the register.
        ///
        /// BATFET off during HIZ mode?
        pub fn set_batfetoff_hiz(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `cmp_en` field of the register.
        ///
        /// Enable Independent Comparator with effective low
        pub fn set_cmp_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `il_avg` field of the register.
        ///
        /// Inductor average current clamp.
        pub fn set_il_avg(&mut self, value: super::IlAvgClamp) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 5, &mut self.bits) };
        }
        ///Write the `otg_vap_mode` field of the register.
        ///
        /// The selection of the external OTG/VAP/FRS pin control. Don't recommend to change pin control after OTG/VAP/FRS pin is pulled high.
        pub fn set_otg_vap_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `en_vbus_vap` field of the register.
        ///
        /// Enable the VBUS VAP for VAP operation mode 2&3
        pub fn set_en_vbus_vap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `batfet_enz` field of the register.
        ///
        /// Turn off BATFET under battery only mode. If charger is not in battery only mode this bit is not allowed to be written to 1. Under battery only OTG mode, this bit is forced to be 0b.
        pub fn set_batfet_enz(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption3A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption3A> for [u8; 1] {
        fn from(val: ChargeOption3A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption3A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption3A");
            d.field("psys_otg_idchg", &self.psys_otg_idchg());
            d.field("batfetoff_hiz", &self.batfetoff_hiz());
            d.field("cmp_en", &self.cmp_en());
            d.field("il_avg", &self.il_avg());
            d.field("otg_vap_mode", &self.otg_vap_mode());
            d.field("en_vbus_vap", &self.en_vbus_vap());
            d.field("batfet_enz", &self.batfet_enz());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption3A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption3A {{ ");
            defmt::write!(f, "psys_otg_idchg: {}, ", &self.psys_otg_idchg());
            defmt::write!(f, "batfetoff_hiz: {=bool}, ", &self.batfetoff_hiz());
            defmt::write!(f, "cmp_en: {=bool}, ", &self.cmp_en());
            defmt::write!(f, "il_avg: {}, ", &self.il_avg());
            defmt::write!(f, "otg_vap_mode: {=bool}, ", &self.otg_vap_mode());
            defmt::write!(f, "en_vbus_vap: {=bool}, ", &self.en_vbus_vap());
            defmt::write!(f, "batfet_enz: {=bool}, ", &self.batfet_enz());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption3A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption3A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption3A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption3A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption3A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption3A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption3A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption3B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption3B {
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
    impl ChargeOption3B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [5] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `en_otg_bigcap` field of the register.
        ///
        /// Enable OTG compensation for VBUS effective capacitance larger than 33μF
        pub fn en_otg_bigcap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_vsys_min_soft_sr` field of the register.
        ///
        /// Enable VSYS_MIN soft slew rate transition
        pub fn en_vsys_min_soft_sr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `en_port_ctrl` field of the register.
        ///
        /// Enable BATFET control
        pub fn en_port_ctrl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `en_ico_mode` field of the register.
        ///
        /// Enable ICO Algorithm.
        pub fn en_ico_mode(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `en_otg` field of the register.
        ///
        /// OTG Mode Enable. Enable device in OTG mode when EN_OTG pin is HIGH.
        pub fn en_otg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `reset_vindpm` field of the register.
        ///
        /// Reset VINDPM Threshold
        pub fn reset_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `reset_reg` field of the register.
        ///
        /// Factory Reset Registers.
        pub fn reset_reg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_hiz` field of the register.
        ///
        /// Device Hi-Z Mode Enable. When the charger is in Hi-Z mode, the device draws minimal quiescent current. With VBUS above UVLO. REGN LDO stays on, and system powers from battery.
        pub fn en_hiz(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `en_otg_bigcap` field of the register.
        ///
        /// Enable OTG compensation for VBUS effective capacitance larger than 33μF
        pub fn set_en_otg_bigcap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_vsys_min_soft_sr` field of the register.
        ///
        /// Enable VSYS_MIN soft slew rate transition
        pub fn set_en_vsys_min_soft_sr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_port_ctrl` field of the register.
        ///
        /// Enable BATFET control
        pub fn set_en_port_ctrl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `en_ico_mode` field of the register.
        ///
        /// Enable ICO Algorithm.
        pub fn set_en_ico_mode(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `en_otg` field of the register.
        ///
        /// OTG Mode Enable. Enable device in OTG mode when EN_OTG pin is HIGH.
        pub fn set_en_otg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `reset_vindpm` field of the register.
        ///
        /// Reset VINDPM Threshold
        pub fn set_reset_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `reset_reg` field of the register.
        ///
        /// Factory Reset Registers.
        pub fn set_reset_reg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_hiz` field of the register.
        ///
        /// Device Hi-Z Mode Enable. When the charger is in Hi-Z mode, the device draws minimal quiescent current. With VBUS above UVLO. REGN LDO stays on, and system powers from battery.
        pub fn set_en_hiz(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption3B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption3B> for [u8; 1] {
        fn from(val: ChargeOption3B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption3B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption3B");
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
    impl defmt::Format for ChargeOption3B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption3B {{ ");
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
    impl core::ops::BitAnd for ChargeOption3B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption3B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption3B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption3B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption3B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption3B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption3B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotOption0A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotOption0A {
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
    impl ProchotOption0A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [129] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `lower_prochot_vindpm` field of the register.
        ///
        /// Enable lower threshold of PROCHOT_VINDPM comparator.
        pub fn lower_prochot_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `inom_deg` field of the register.
        ///
        /// INOM deglitch time.
        pub fn inom_deg(&self) -> super::InomDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vsys_th_1` field of the register.
        ///
        /// VSYS Threshold to trigger discharging VBUS in VAP mode. Measure on VSYS with fixed 5-µs deglitch time. Trigger when SYS pin voltage is below the thresholds. There is a fixed DC offset which is 3.2 V. 2S - 4s battery (Default 6.4 V)
        pub fn vsys_th_1(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 8) };
            raw
        }
        ///Write the `lower_prochot_vindpm` field of the register.
        ///
        /// Enable lower threshold of PROCHOT_VINDPM comparator.
        pub fn set_lower_prochot_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `inom_deg` field of the register.
        ///
        /// INOM deglitch time.
        pub fn set_inom_deg(&mut self, value: super::InomDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `vsys_th_1` field of the register.
        ///
        /// VSYS Threshold to trigger discharging VBUS in VAP mode. Measure on VSYS with fixed 5-µs deglitch time. Trigger when SYS pin voltage is below the thresholds. There is a fixed DC offset which is 3.2 V. 2S - 4s battery (Default 6.4 V)
        pub fn set_vsys_th_1(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotOption0A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotOption0A> for [u8; 1] {
        fn from(val: ProchotOption0A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotOption0A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotOption0A");
            d.field("lower_prochot_vindpm", &self.lower_prochot_vindpm());
            d.field("inom_deg", &self.inom_deg());
            d.field("vsys_th_1", &self.vsys_th_1());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotOption0A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotOption0A {{ ");
            defmt::write!(f, "lower_prochot_vindpm: {=bool}, ", &self.lower_prochot_vindpm());
            defmt::write!(f, "inom_deg: {}, ", &self.inom_deg());
            defmt::write!(f, "vsys_th_1: {=u8}, ", &self.vsys_th_1());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotOption0A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotOption0A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotOption0A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotOption0A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotOption0A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotOption0A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotOption0A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotOption0B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotOption0B {
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
    impl ProchotOption0B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [74] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `prochot_vindpm_80_90` field of the register.
        ///
        /// Lower threshold of the PROCHOT_VINDPM comparator. When LOWER_PROCHOT_VINDPM=1, the threshold of PROCHOT_VINDPM is determined by this setting.
        pub fn prochot_vindpm_80_90(&self) -> super::Threshold {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `icrit_deg` field of the register.
        ///
        /// ICRIT deglitch time to trigger PROCHOT.
        pub fn icrit_deg(&self) -> super::IcritDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 3) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `ilim_2_vth` field of the register.
        ///
        /// ILIM2 Threshold.
        pub fn ilim_2_vth(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 8) };
            raw
        }
        ///Write the `prochot_vindpm_80_90` field of the register.
        ///
        /// Lower threshold of the PROCHOT_VINDPM comparator. When LOWER_PROCHOT_VINDPM=1, the threshold of PROCHOT_VINDPM is determined by this setting.
        pub fn set_prochot_vindpm_80_90(&mut self, value: super::Threshold) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `icrit_deg` field of the register.
        ///
        /// ICRIT deglitch time to trigger PROCHOT.
        pub fn set_icrit_deg(&mut self, value: super::IcritDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 3, &mut self.bits) };
        }
        ///Write the `ilim_2_vth` field of the register.
        ///
        /// ILIM2 Threshold.
        pub fn set_ilim_2_vth(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotOption0B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotOption0B> for [u8; 1] {
        fn from(val: ProchotOption0B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotOption0B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotOption0B");
            d.field("prochot_vindpm_80_90", &self.prochot_vindpm_80_90());
            d.field("icrit_deg", &self.icrit_deg());
            d.field("ilim_2_vth", &self.ilim_2_vth());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotOption0B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotOption0B {{ ");
            defmt::write!(f, "prochot_vindpm_80_90: {}, ", &self.prochot_vindpm_80_90());
            defmt::write!(f, "icrit_deg: {}, ", &self.icrit_deg());
            defmt::write!(f, "ilim_2_vth: {=u8}, ", &self.ilim_2_vth());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotOption0B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotOption0B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotOption0B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotOption0B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotOption0B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotOption0B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotOption0B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotOption1A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotOption1A {
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
    impl ProchotOption1A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [160] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `pp_acok` field of the register.
        ///
        /// Adapter removal PROCHOT profile enable.
        pub fn pp_acok(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `pp_batpres` field of the register.
        ///
        /// Battery removal PROCHOT profile enable.
        pub fn pp_batpres(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `pp_vsys` field of the register.
        ///
        /// VSYS PROCHOT profile enable.
        pub fn pp_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `pp_idchg_1` field of the register.
        ///
        /// IDCHG1 PROCHOT profile enable.
        pub fn pp_idchg_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `pp_inom` field of the register.
        ///
        /// INOM PROCHOT profile enable.
        pub fn pp_inom(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `pp_icrit` field of the register.
        ///
        /// ICRIT PROCHOT profile enable.
        pub fn pp_icrit(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `pp_comp` field of the register.
        ///
        /// COMP PROCHOT profile enable.
        pub fn pp_comp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `pp_vindpm` field of the register.
        ///
        /// VINDPM PROCHOT profile enable.
        pub fn pp_vindpm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `pp_acok` field of the register.
        ///
        /// Adapter removal PROCHOT profile enable.
        pub fn set_pp_acok(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `pp_batpres` field of the register.
        ///
        /// Battery removal PROCHOT profile enable.
        pub fn set_pp_batpres(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `pp_vsys` field of the register.
        ///
        /// VSYS PROCHOT profile enable.
        pub fn set_pp_vsys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `pp_idchg_1` field of the register.
        ///
        /// IDCHG1 PROCHOT profile enable.
        pub fn set_pp_idchg_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `pp_inom` field of the register.
        ///
        /// INOM PROCHOT profile enable.
        pub fn set_pp_inom(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `pp_icrit` field of the register.
        ///
        /// ICRIT PROCHOT profile enable.
        pub fn set_pp_icrit(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `pp_comp` field of the register.
        ///
        /// COMP PROCHOT profile enable.
        pub fn set_pp_comp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `pp_vindpm` field of the register.
        ///
        /// VINDPM PROCHOT profile enable.
        pub fn set_pp_vindpm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotOption1A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotOption1A> for [u8; 1] {
        fn from(val: ProchotOption1A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotOption1A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotOption1A");
            d.field("pp_acok", &self.pp_acok());
            d.field("pp_batpres", &self.pp_batpres());
            d.field("pp_vsys", &self.pp_vsys());
            d.field("pp_idchg_1", &self.pp_idchg_1());
            d.field("pp_inom", &self.pp_inom());
            d.field("pp_icrit", &self.pp_icrit());
            d.field("pp_comp", &self.pp_comp());
            d.field("pp_vindpm", &self.pp_vindpm());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotOption1A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotOption1A {{ ");
            defmt::write!(f, "pp_acok: {=bool}, ", &self.pp_acok());
            defmt::write!(f, "pp_batpres: {=bool}, ", &self.pp_batpres());
            defmt::write!(f, "pp_vsys: {=bool}, ", &self.pp_vsys());
            defmt::write!(f, "pp_idchg_1: {=bool}, ", &self.pp_idchg_1());
            defmt::write!(f, "pp_inom: {=bool}, ", &self.pp_inom());
            defmt::write!(f, "pp_icrit: {=bool}, ", &self.pp_icrit());
            defmt::write!(f, "pp_comp: {=bool}, ", &self.pp_comp());
            defmt::write!(f, "pp_vindpm: {=bool}, ", &self.pp_vindpm());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotOption1A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotOption1A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotOption1A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotOption1A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotOption1A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotOption1A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotOption1A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProchotOption1B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ProchotOption1B {
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
    impl ProchotOption1B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [65] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `idchg_deg_1` field of the register.
        ///
        /// IDCHG deglitch time.
        pub fn idchg_deg_1(&self) -> super::IdchgDeglitchTime {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 2) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `idchg_th_1` field of the register.
        ///
        /// IDCHG level 1 Threshold.
        pub fn idchg_th_1(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 8) };
            raw
        }
        ///Write the `idchg_deg_1` field of the register.
        ///
        /// IDCHG deglitch time.
        pub fn set_idchg_deg_1(&mut self, value: super::IdchgDeglitchTime) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 2, &mut self.bits) };
        }
        ///Write the `idchg_th_1` field of the register.
        ///
        /// IDCHG level 1 Threshold.
        pub fn set_idchg_th_1(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ProchotOption1B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ProchotOption1B> for [u8; 1] {
        fn from(val: ProchotOption1B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ProchotOption1B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ProchotOption1B");
            d.field("idchg_deg_1", &self.idchg_deg_1());
            d.field("idchg_th_1", &self.idchg_th_1());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ProchotOption1B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ProchotOption1B {{ ");
            defmt::write!(f, "idchg_deg_1: {}, ", &self.idchg_deg_1());
            defmt::write!(f, "idchg_th_1: {=u8}, ", &self.idchg_th_1());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ProchotOption1B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ProchotOption1B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ProchotOption1B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ProchotOption1B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ProchotOption1B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ProchotOption1B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ProchotOption1B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcOptionA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcOptionA {
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
    impl AdcOptionA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `en_adc_vbat` field of the register.
        ///
        /// Enable SRN pin Voltage ADC Channel.
        pub fn en_adc_vbat(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_adc_vsys` field of the register.
        ///
        /// Enable VSYS pin Voltage ADC Channel.
        pub fn en_adc_vsys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `en_adc_ichg` field of the register.
        ///
        /// Enable ICHG ADC Channel.
        pub fn en_adc_ichg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `en_adc_idchg` field of the register.
        ///
        /// Enable IDCHG ADC Channel.
        pub fn en_adc_idchg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `en_adc_iin` field of the register.
        ///
        /// Enable IIN ADC Channel.
        pub fn en_adc_iin(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `en_adc_psys` field of the register.
        ///
        /// Enable PSYS pin Voltage ADC Channel.
        pub fn en_adc_psys(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `en_adc_vbus` field of the register.
        ///
        /// Enable VBUS pin Voltage ADC Channel.
        pub fn en_adc_vbus(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `en_adc_cmpin` field of the register.
        ///
        /// Enable CMPIN_TR pin Voltage ADC Channel.
        pub fn en_adc_cmpin(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `en_adc_vbat` field of the register.
        ///
        /// Enable SRN pin Voltage ADC Channel.
        pub fn set_en_adc_vbat(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_adc_vsys` field of the register.
        ///
        /// Enable VSYS pin Voltage ADC Channel.
        pub fn set_en_adc_vsys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `en_adc_ichg` field of the register.
        ///
        /// Enable ICHG ADC Channel.
        pub fn set_en_adc_ichg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `en_adc_idchg` field of the register.
        ///
        /// Enable IDCHG ADC Channel.
        pub fn set_en_adc_idchg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `en_adc_iin` field of the register.
        ///
        /// Enable IIN ADC Channel.
        pub fn set_en_adc_iin(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `en_adc_psys` field of the register.
        ///
        /// Enable PSYS pin Voltage ADC Channel.
        pub fn set_en_adc_psys(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `en_adc_vbus` field of the register.
        ///
        /// Enable VBUS pin Voltage ADC Channel.
        pub fn set_en_adc_vbus(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `en_adc_cmpin` field of the register.
        ///
        /// Enable CMPIN_TR pin Voltage ADC Channel.
        pub fn set_en_adc_cmpin(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for AdcOptionA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcOptionA> for [u8; 1] {
        fn from(val: AdcOptionA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcOptionA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcOptionA");
            d.field("en_adc_vbat", &self.en_adc_vbat());
            d.field("en_adc_vsys", &self.en_adc_vsys());
            d.field("en_adc_ichg", &self.en_adc_ichg());
            d.field("en_adc_idchg", &self.en_adc_idchg());
            d.field("en_adc_iin", &self.en_adc_iin());
            d.field("en_adc_psys", &self.en_adc_psys());
            d.field("en_adc_vbus", &self.en_adc_vbus());
            d.field("en_adc_cmpin", &self.en_adc_cmpin());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcOptionA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcOptionA {{ ");
            defmt::write!(f, "en_adc_vbat: {=bool}, ", &self.en_adc_vbat());
            defmt::write!(f, "en_adc_vsys: {=bool}, ", &self.en_adc_vsys());
            defmt::write!(f, "en_adc_ichg: {=bool}, ", &self.en_adc_ichg());
            defmt::write!(f, "en_adc_idchg: {=bool}, ", &self.en_adc_idchg());
            defmt::write!(f, "en_adc_iin: {=bool}, ", &self.en_adc_iin());
            defmt::write!(f, "en_adc_psys: {=bool}, ", &self.en_adc_psys());
            defmt::write!(f, "en_adc_vbus: {=bool}, ", &self.en_adc_vbus());
            defmt::write!(f, "en_adc_cmpin: {=bool}, ", &self.en_adc_cmpin());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcOptionA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcOptionA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcOptionA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcOptionA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcOptionA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcOptionA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcOptionA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcOptionB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for AdcOptionB {
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
    impl AdcOptionB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [32] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `adc_full_scale` field of the register.
        ///
        /// ADC input voltage range. When input voltage is below 5 V, or battery is 1S, full scale 2.04 V is recommended. 0b 2.04 V 1b 3.06 V <default at POR>
        pub fn adc_full_scale(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `adc_start` field of the register.
        ///
        /// ADC start control. After the one-shot update is complete, this bit automatically resets to zero
        pub fn adc_start(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `adc_conv` field of the register.
        ///
        /// Typical ADC conversion time is 10 ms. 0b OneShot 1b Continuous
        pub fn adc_conv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Write the `adc_full_scale` field of the register.
        ///
        /// ADC input voltage range. When input voltage is below 5 V, or battery is 1S, full scale 2.04 V is recommended. 0b 2.04 V 1b 3.06 V <default at POR>
        pub fn set_adc_full_scale(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `adc_start` field of the register.
        ///
        /// ADC start control. After the one-shot update is complete, this bit automatically resets to zero
        pub fn set_adc_start(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `adc_conv` field of the register.
        ///
        /// Typical ADC conversion time is 10 ms. 0b OneShot 1b Continuous
        pub fn set_adc_conv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for AdcOptionB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<AdcOptionB> for [u8; 1] {
        fn from(val: AdcOptionB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for AdcOptionB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("AdcOptionB");
            d.field("adc_full_scale", &self.adc_full_scale());
            d.field("adc_start", &self.adc_start());
            d.field("adc_conv", &self.adc_conv());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for AdcOptionB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AdcOptionB {{ ");
            defmt::write!(f, "adc_full_scale: {=bool}, ", &self.adc_full_scale());
            defmt::write!(f, "adc_start: {=bool}, ", &self.adc_start());
            defmt::write!(f, "adc_conv: {=bool}, ", &self.adc_conv());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for AdcOptionB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for AdcOptionB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for AdcOptionB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for AdcOptionB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for AdcOptionB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for AdcOptionB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for AdcOptionB {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption4A {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption4A {
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
    impl ChargeOption4A {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [72] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `stat_ptm` field of the register.
        ///
        /// PTM operation status active.
        pub fn stat_ptm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `stat_idchg_2` field of the register.
        ///
        /// IDCHG2 status triggered.
        pub fn stat_idchg_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `pp_idchg_2` field of the register.
        ///
        /// Enable IDCHG_TH2 PROCHOT Profile.
        pub fn pp_idchg_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `idchg_th_2` field of the register.
        ///
        /// Battery discharge current limit2 based on percentage of IDCHG_TH1.
        pub fn idchg_th_2(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 6) };
            raw
        }
        ///Read the `idchg_deg_2` field of the register.
        ///
        /// Battery discharge current limit 2 deglitch time.
        pub fn idchg_deg_2(&self) -> super::IdchgDeglitchTime2 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 8) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Write the `stat_ptm` field of the register.
        ///
        /// PTM operation status active.
        pub fn set_stat_ptm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `stat_idchg_2` field of the register.
        ///
        /// IDCHG2 status triggered.
        pub fn set_stat_idchg_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `pp_idchg_2` field of the register.
        ///
        /// Enable IDCHG_TH2 PROCHOT Profile.
        pub fn set_pp_idchg_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `idchg_th_2` field of the register.
        ///
        /// Battery discharge current limit2 based on percentage of IDCHG_TH1.
        pub fn set_idchg_th_2(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 6, &mut self.bits) };
        }
        ///Write the `idchg_deg_2` field of the register.
        ///
        /// Battery discharge current limit 2 deglitch time.
        pub fn set_idchg_deg_2(&mut self, value: super::IdchgDeglitchTime2) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption4A {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption4A> for [u8; 1] {
        fn from(val: ChargeOption4A) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption4A {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption4A");
            d.field("stat_ptm", &self.stat_ptm());
            d.field("stat_idchg_2", &self.stat_idchg_2());
            d.field("pp_idchg_2", &self.pp_idchg_2());
            d.field("idchg_th_2", &self.idchg_th_2());
            d.field("idchg_deg_2", &self.idchg_deg_2());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption4A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption4A {{ ");
            defmt::write!(f, "stat_ptm: {=bool}, ", &self.stat_ptm());
            defmt::write!(f, "stat_idchg_2: {=bool}, ", &self.stat_idchg_2());
            defmt::write!(f, "pp_idchg_2: {=bool}, ", &self.pp_idchg_2());
            defmt::write!(f, "idchg_th_2: {=u8}, ", &self.idchg_th_2());
            defmt::write!(f, "idchg_deg_2: {}, ", &self.idchg_deg_2());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption4A {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption4A {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption4A {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption4A {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption4A {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption4A {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption4A {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargeOption4B {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ChargeOption4B {
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
    impl ChargeOption4B {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `stat_vbus_vap` field of the register.
        ///
        /// VBUS_VAP status triggered.
        pub fn stat_vbus_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `pp_vbus_vap` field of the register.
        ///
        /// Enable VBUS_VAP PROCHOT Profile.
        pub fn pp_vbus_vap(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `vsys_uvp_no_hiccup` field of the register.
        ///
        /// Disable VSYS_UVP Hiccup mode operation.
        pub fn vsys_uvp_no_hiccup(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `en_dither` field of the register.
        ///
        /// Frequency Dither configuration.
        pub fn en_dither(&self) -> super::DitherConfig {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 5) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `vsys_uvp` field of the register.
        ///
        /// VSYS Under Voltage Lock Out. After UVP is triggered the charger enters hiccup mode, and then the charger is latched off if the restart fails 7 times in 90s.
        pub fn vsys_uvp(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 8) };
            raw
        }
        ///Write the `stat_vbus_vap` field of the register.
        ///
        /// VBUS_VAP status triggered.
        pub fn set_stat_vbus_vap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `pp_vbus_vap` field of the register.
        ///
        /// Enable VBUS_VAP PROCHOT Profile.
        pub fn set_pp_vbus_vap(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `vsys_uvp_no_hiccup` field of the register.
        ///
        /// Disable VSYS_UVP Hiccup mode operation.
        pub fn set_vsys_uvp_no_hiccup(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `en_dither` field of the register.
        ///
        /// Frequency Dither configuration.
        pub fn set_en_dither(&mut self, value: super::DitherConfig) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 5, &mut self.bits) };
        }
        ///Write the `vsys_uvp` field of the register.
        ///
        /// VSYS Under Voltage Lock Out. After UVP is triggered the charger enters hiccup mode, and then the charger is latched off if the restart fails 7 times in 90s.
        pub fn set_vsys_uvp(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ChargeOption4B {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ChargeOption4B> for [u8; 1] {
        fn from(val: ChargeOption4B) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ChargeOption4B {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ChargeOption4B");
            d.field("stat_vbus_vap", &self.stat_vbus_vap());
            d.field("pp_vbus_vap", &self.pp_vbus_vap());
            d.field("vsys_uvp_no_hiccup", &self.vsys_uvp_no_hiccup());
            d.field("en_dither", &self.en_dither());
            d.field("vsys_uvp", &self.vsys_uvp());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ChargeOption4B {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ChargeOption4B {{ ");
            defmt::write!(f, "stat_vbus_vap: {=bool}, ", &self.stat_vbus_vap());
            defmt::write!(f, "pp_vbus_vap: {=bool}, ", &self.pp_vbus_vap());
            defmt::write!(f, "vsys_uvp_no_hiccup: {=bool}, ", &self.vsys_uvp_no_hiccup());
            defmt::write!(f, "en_dither: {}, ", &self.en_dither());
            defmt::write!(f, "vsys_uvp: {=u8}, ", &self.vsys_uvp());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ChargeOption4B {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ChargeOption4B {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ChargeOption4B {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ChargeOption4B {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ChargeOption4B {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ChargeOption4B {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ChargeOption4B {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VminActiveProtectionA {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for VminActiveProtectionA {
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
    impl VminActiveProtectionA {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [108] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `en_frs` field of the register.
        ///
        /// Fast Role Swap Feature Enable.
        pub fn en_frs(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `en_vsysth_2_follow_vsysth_1` field of the register.
        ///
        /// Enable internal VSYS_TH2 follow VSYS_TH1 setting neglecting register VSYS_TH2 setting.
        pub fn en_vsysth_2_follow_vsysth_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `vsys_th_2` field of the register.
        ///
        /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
        pub fn vsys_th_2(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 8) };
            raw
        }
        ///Write the `en_frs` field of the register.
        ///
        /// Fast Role Swap Feature Enable.
        pub fn set_en_frs(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `en_vsysth_2_follow_vsysth_1` field of the register.
        ///
        /// Enable internal VSYS_TH2 follow VSYS_TH1 setting neglecting register VSYS_TH2 setting.
        pub fn set_en_vsysth_2_follow_vsysth_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `vsys_th_2` field of the register.
        ///
        /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
        pub fn set_vsys_th_2(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for VminActiveProtectionA {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<VminActiveProtectionA> for [u8; 1] {
        fn from(val: VminActiveProtectionA) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VminActiveProtectionA {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VminActiveProtectionA");
            d.field("en_frs", &self.en_frs());
            d.field("en_vsysth_2_follow_vsysth_1", &self.en_vsysth_2_follow_vsysth_1());
            d.field("vsys_th_2", &self.vsys_th_2());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for VminActiveProtectionA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VminActiveProtectionA {{ ");
            defmt::write!(f, "en_frs: {=bool}, ", &self.en_frs());
            defmt::write!(
                f,
                "en_vsysth_2_follow_vsysth_1: {=bool}, ",
                &self.en_vsysth_2_follow_vsysth_1()
            );
            defmt::write!(f, "vsys_th_2: {=u8}, ", &self.vsys_th_2());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for VminActiveProtectionA {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VminActiveProtectionA {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VminActiveProtectionA {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VminActiveProtectionA {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VminActiveProtectionA {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VminActiveProtectionA {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VminActiveProtectionA {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VminActiveProtectionB {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for VminActiveProtectionB {
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
    impl VminActiveProtectionB {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `vbus_vap_th` field of the register.
        ///
        /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
        pub fn vbus_vap_th(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 8) };
            raw
        }
        ///Write the `vbus_vap_th` field of the register.
        ///
        /// VAP Mode2 VBUS /PROCHOT trigger voltage threshold.
        pub fn set_vbus_vap_th(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for VminActiveProtectionB {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<VminActiveProtectionB> for [u8; 1] {
        fn from(val: VminActiveProtectionB) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for VminActiveProtectionB {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("VminActiveProtectionB");
            d.field("vbus_vap_th", &self.vbus_vap_th());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for VminActiveProtectionB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VminActiveProtectionB {{ ");
            defmt::write!(f, "vbus_vap_th: {=u8}, ", &self.vbus_vap_th());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for VminActiveProtectionB {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for VminActiveProtectionB {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for VminActiveProtectionB {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for VminActiveProtectionB {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for VminActiveProtectionB {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for VminActiveProtectionB {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for VminActiveProtectionB {
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
            Self { bits: [9, 196] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `input_voltage` field of the register.
        ///
        /// OTG Output voltage limit
        pub fn input_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 2, 14) };
            raw
        }
        ///Write the `input_voltage` field of the register.
        ///
        /// OTG Output voltage limit
        pub fn set_input_voltage(&mut self, value: u16) {
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
            d.field("input_voltage", &self.input_voltage());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for OtgVoltage {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OtgVoltage {{ ");
            defmt::write!(f, "input_voltage: {=u16}, ", &self.input_voltage());
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
            Self { bits: [60, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `otg_current` field of the register.
        ///
        /// OTG output current limit
        pub fn otg_current(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 15) };
            raw
        }
        ///Write the `otg_current` field of the register.
        ///
        /// OTG output current limit
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
        /// Input Voltage Limit
        pub fn input_voltage(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 14) };
            raw
        }
        ///Write the `input_voltage` field of the register.
        ///
        /// Input Voltage Limit
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
        ///Read the `charge_voltage` field of the register.
        ///
        /// Charge voltage setting.
        pub fn charge_voltage(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 16) };
            raw
        }
        ///Write the `charge_voltage` field of the register.
        ///
        /// Charge voltage setting.
        pub fn set_charge_voltage(&mut self, value: u8) {
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
            d.field("charge_voltage", &self.charge_voltage());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for VsysMin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VsysMin {{ ");
            defmt::write!(f, "charge_voltage: {=u8}, ", &self.charge_voltage());
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
        /// Maximum input current limit
        pub fn iin_host(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 15) };
            raw
        }
        ///Write the `iin_host` field of the register.
        ///
        /// Maximum input current limit
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
    pub struct ManufactureId {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for ManufactureId {
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
    impl ManufactureId {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [64] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `manufacture_id` field of the register.
        ///
        /// Manufacture ID.
        pub fn manufacture_id(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Write the `manufacture_id` field of the register.
        ///
        /// Manufacture ID.
        pub fn set_manufacture_id(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for ManufactureId {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<ManufactureId> for [u8; 1] {
        fn from(val: ManufactureId) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for ManufactureId {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("ManufactureId");
            d.field("manufacture_id", &self.manufacture_id());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for ManufactureId {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ManufactureId {{ ");
            defmt::write!(f, "manufacture_id: {=u8}, ", &self.manufacture_id());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for ManufactureId {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for ManufactureId {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for ManufactureId {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for ManufactureId {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for ManufactureId {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for ManufactureId {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for ManufactureId {
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
        /// Device ID.
        pub fn device_id(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Write the `device_id` field of the register.
        ///
        /// Device ID.
        pub fn set_device_id(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
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
    /// Enum containing all possible field set types
    pub enum FieldSetValue {
        ChargeOption0A(ChargeOption0A),
        ChargeOption0B(ChargeOption0B),
        ChargeCurrent(ChargeCurrent),
        ChargeVoltage(ChargeVoltage),
        ChargerStatus0A(ChargerStatus0A),
        ChargerStatus0B(ChargerStatus0B),
        ProchotStatusRegA(ProchotStatusRegA),
        ProchotStatusRegB(ProchotStatusRegB),
        IinDpm(IinDpm),
        AdcPsys(AdcPsys),
        AdcVbus(AdcVbus),
        AdcIbatDchrg(AdcIbatDchrg),
        AdcIbatChrg(AdcIbatChrg),
        AdcCmpin(AdcCmpin),
        AdcVbat(AdcVbat),
        AdcVsys(AdcVsys),
        ChargeOption1A(ChargeOption1A),
        ChargeOption1B(ChargeOption1B),
        ChargeOption2A(ChargeOption2A),
        ChargeOption2B(ChargeOption2B),
        ChargeOption3A(ChargeOption3A),
        ChargeOption3B(ChargeOption3B),
        ProchotOption0A(ProchotOption0A),
        ProchotOption0B(ProchotOption0B),
        ProchotOption1A(ProchotOption1A),
        ProchotOption1B(ProchotOption1B),
        AdcOptionA(AdcOptionA),
        AdcOptionB(AdcOptionB),
        ChargeOption4A(ChargeOption4A),
        ChargeOption4B(ChargeOption4B),
        VminActiveProtectionA(VminActiveProtectionA),
        VminActiveProtectionB(VminActiveProtectionB),
        OtgVoltage(OtgVoltage),
        OtgCurrent(OtgCurrent),
        InputVoltage(InputVoltage),
        VsysMin(VsysMin),
        IinHost(IinHost),
        ManufactureId(ManufactureId),
        DeviceId(DeviceId),
    }
    impl core::fmt::Debug for FieldSetValue {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::ChargeOption0A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption0B(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeCurrent(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeVoltage(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargerStatus0A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargerStatus0B(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotStatusRegA(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotStatusRegB(val) => core::fmt::Debug::fmt(val, f),
                Self::IinDpm(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcPsys(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcVbus(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcIbatDchrg(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcIbatChrg(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcCmpin(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcVbat(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcVsys(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption1A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption1B(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption2A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption2B(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption3A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption3B(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotOption0A(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotOption0B(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotOption1A(val) => core::fmt::Debug::fmt(val, f),
                Self::ProchotOption1B(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcOptionA(val) => core::fmt::Debug::fmt(val, f),
                Self::AdcOptionB(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption4A(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargeOption4B(val) => core::fmt::Debug::fmt(val, f),
                Self::VminActiveProtectionA(val) => core::fmt::Debug::fmt(val, f),
                Self::VminActiveProtectionB(val) => core::fmt::Debug::fmt(val, f),
                Self::OtgVoltage(val) => core::fmt::Debug::fmt(val, f),
                Self::OtgCurrent(val) => core::fmt::Debug::fmt(val, f),
                Self::InputVoltage(val) => core::fmt::Debug::fmt(val, f),
                Self::VsysMin(val) => core::fmt::Debug::fmt(val, f),
                Self::IinHost(val) => core::fmt::Debug::fmt(val, f),
                Self::ManufactureId(val) => core::fmt::Debug::fmt(val, f),
                Self::DeviceId(val) => core::fmt::Debug::fmt(val, f),
                #[allow(unreachable_patterns)]
                _ => unreachable!(),
            }
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for FieldSetValue {
        fn format(&self, f: defmt::Formatter) {
            match self {
                Self::ChargeOption0A(val) => defmt::Format::format(val, f),
                Self::ChargeOption0B(val) => defmt::Format::format(val, f),
                Self::ChargeCurrent(val) => defmt::Format::format(val, f),
                Self::ChargeVoltage(val) => defmt::Format::format(val, f),
                Self::ChargerStatus0A(val) => defmt::Format::format(val, f),
                Self::ChargerStatus0B(val) => defmt::Format::format(val, f),
                Self::ProchotStatusRegA(val) => defmt::Format::format(val, f),
                Self::ProchotStatusRegB(val) => defmt::Format::format(val, f),
                Self::IinDpm(val) => defmt::Format::format(val, f),
                Self::AdcPsys(val) => defmt::Format::format(val, f),
                Self::AdcVbus(val) => defmt::Format::format(val, f),
                Self::AdcIbatDchrg(val) => defmt::Format::format(val, f),
                Self::AdcIbatChrg(val) => defmt::Format::format(val, f),
                Self::AdcCmpin(val) => defmt::Format::format(val, f),
                Self::AdcVbat(val) => defmt::Format::format(val, f),
                Self::AdcVsys(val) => defmt::Format::format(val, f),
                Self::ChargeOption1A(val) => defmt::Format::format(val, f),
                Self::ChargeOption1B(val) => defmt::Format::format(val, f),
                Self::ChargeOption2A(val) => defmt::Format::format(val, f),
                Self::ChargeOption2B(val) => defmt::Format::format(val, f),
                Self::ChargeOption3A(val) => defmt::Format::format(val, f),
                Self::ChargeOption3B(val) => defmt::Format::format(val, f),
                Self::ProchotOption0A(val) => defmt::Format::format(val, f),
                Self::ProchotOption0B(val) => defmt::Format::format(val, f),
                Self::ProchotOption1A(val) => defmt::Format::format(val, f),
                Self::ProchotOption1B(val) => defmt::Format::format(val, f),
                Self::AdcOptionA(val) => defmt::Format::format(val, f),
                Self::AdcOptionB(val) => defmt::Format::format(val, f),
                Self::ChargeOption4A(val) => defmt::Format::format(val, f),
                Self::ChargeOption4B(val) => defmt::Format::format(val, f),
                Self::VminActiveProtectionA(val) => defmt::Format::format(val, f),
                Self::VminActiveProtectionB(val) => defmt::Format::format(val, f),
                Self::OtgVoltage(val) => defmt::Format::format(val, f),
                Self::OtgCurrent(val) => defmt::Format::format(val, f),
                Self::InputVoltage(val) => defmt::Format::format(val, f),
                Self::VsysMin(val) => defmt::Format::format(val, f),
                Self::IinHost(val) => defmt::Format::format(val, f),
                Self::ManufactureId(val) => defmt::Format::format(val, f),
                Self::DeviceId(val) => defmt::Format::format(val, f),
            }
        }
    }
    impl From<ChargeOption0A> for FieldSetValue {
        fn from(val: ChargeOption0A) -> Self {
            Self::ChargeOption0A(val)
        }
    }
    impl From<ChargeOption0B> for FieldSetValue {
        fn from(val: ChargeOption0B) -> Self {
            Self::ChargeOption0B(val)
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
    impl From<ChargerStatus0A> for FieldSetValue {
        fn from(val: ChargerStatus0A) -> Self {
            Self::ChargerStatus0A(val)
        }
    }
    impl From<ChargerStatus0B> for FieldSetValue {
        fn from(val: ChargerStatus0B) -> Self {
            Self::ChargerStatus0B(val)
        }
    }
    impl From<ProchotStatusRegA> for FieldSetValue {
        fn from(val: ProchotStatusRegA) -> Self {
            Self::ProchotStatusRegA(val)
        }
    }
    impl From<ProchotStatusRegB> for FieldSetValue {
        fn from(val: ProchotStatusRegB) -> Self {
            Self::ProchotStatusRegB(val)
        }
    }
    impl From<IinDpm> for FieldSetValue {
        fn from(val: IinDpm) -> Self {
            Self::IinDpm(val)
        }
    }
    impl From<AdcPsys> for FieldSetValue {
        fn from(val: AdcPsys) -> Self {
            Self::AdcPsys(val)
        }
    }
    impl From<AdcVbus> for FieldSetValue {
        fn from(val: AdcVbus) -> Self {
            Self::AdcVbus(val)
        }
    }
    impl From<AdcIbatDchrg> for FieldSetValue {
        fn from(val: AdcIbatDchrg) -> Self {
            Self::AdcIbatDchrg(val)
        }
    }
    impl From<AdcIbatChrg> for FieldSetValue {
        fn from(val: AdcIbatChrg) -> Self {
            Self::AdcIbatChrg(val)
        }
    }
    impl From<AdcCmpin> for FieldSetValue {
        fn from(val: AdcCmpin) -> Self {
            Self::AdcCmpin(val)
        }
    }
    impl From<AdcVbat> for FieldSetValue {
        fn from(val: AdcVbat) -> Self {
            Self::AdcVbat(val)
        }
    }
    impl From<AdcVsys> for FieldSetValue {
        fn from(val: AdcVsys) -> Self {
            Self::AdcVsys(val)
        }
    }
    impl From<ChargeOption1A> for FieldSetValue {
        fn from(val: ChargeOption1A) -> Self {
            Self::ChargeOption1A(val)
        }
    }
    impl From<ChargeOption1B> for FieldSetValue {
        fn from(val: ChargeOption1B) -> Self {
            Self::ChargeOption1B(val)
        }
    }
    impl From<ChargeOption2A> for FieldSetValue {
        fn from(val: ChargeOption2A) -> Self {
            Self::ChargeOption2A(val)
        }
    }
    impl From<ChargeOption2B> for FieldSetValue {
        fn from(val: ChargeOption2B) -> Self {
            Self::ChargeOption2B(val)
        }
    }
    impl From<ChargeOption3A> for FieldSetValue {
        fn from(val: ChargeOption3A) -> Self {
            Self::ChargeOption3A(val)
        }
    }
    impl From<ChargeOption3B> for FieldSetValue {
        fn from(val: ChargeOption3B) -> Self {
            Self::ChargeOption3B(val)
        }
    }
    impl From<ProchotOption0A> for FieldSetValue {
        fn from(val: ProchotOption0A) -> Self {
            Self::ProchotOption0A(val)
        }
    }
    impl From<ProchotOption0B> for FieldSetValue {
        fn from(val: ProchotOption0B) -> Self {
            Self::ProchotOption0B(val)
        }
    }
    impl From<ProchotOption1A> for FieldSetValue {
        fn from(val: ProchotOption1A) -> Self {
            Self::ProchotOption1A(val)
        }
    }
    impl From<ProchotOption1B> for FieldSetValue {
        fn from(val: ProchotOption1B) -> Self {
            Self::ProchotOption1B(val)
        }
    }
    impl From<AdcOptionA> for FieldSetValue {
        fn from(val: AdcOptionA) -> Self {
            Self::AdcOptionA(val)
        }
    }
    impl From<AdcOptionB> for FieldSetValue {
        fn from(val: AdcOptionB) -> Self {
            Self::AdcOptionB(val)
        }
    }
    impl From<ChargeOption4A> for FieldSetValue {
        fn from(val: ChargeOption4A) -> Self {
            Self::ChargeOption4A(val)
        }
    }
    impl From<ChargeOption4B> for FieldSetValue {
        fn from(val: ChargeOption4B) -> Self {
            Self::ChargeOption4B(val)
        }
    }
    impl From<VminActiveProtectionA> for FieldSetValue {
        fn from(val: VminActiveProtectionA) -> Self {
            Self::VminActiveProtectionA(val)
        }
    }
    impl From<VminActiveProtectionB> for FieldSetValue {
        fn from(val: VminActiveProtectionB) -> Self {
            Self::VminActiveProtectionB(val)
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
    impl From<ManufactureId> for FieldSetValue {
        fn from(val: ManufactureId) -> Self {
            Self::ManufactureId(val)
        }
    }
    impl From<DeviceId> for FieldSetValue {
        fn from(val: DeviceId) -> Self {
            Self::DeviceId(val)
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IbatGain {
    EightX = 0,
    SixtyFourX = 1,
}
impl core::convert::TryFrom<u8> for IbatGain {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::EightX),
            1 => Ok(Self::SixtyFourX),
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
            IbatGain::SixtyFourX => 1,
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
    EightHundredkHz = 0,
    SixHundredkHz = 1,
}
impl core::convert::TryFrom<u8> for SwitchingFreq {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::EightHundredkHz),
            1 => Ok(Self::SixHundredkHz),
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
            SwitchingFreq::EightHundredkHz => 0,
            SwitchingFreq::SixHundredkHz => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum MaxDelay {
    Disable = 0,
    FiveSeconds = 1,
    EightyEightSeconds = 2,
    OneHundredSeventyFiveSeconds = 3,
}
impl core::convert::TryFrom<u8> for MaxDelay {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disable),
            1 => Ok(Self::FiveSeconds),
            2 => Ok(Self::EightyEightSeconds),
            3 => Ok(Self::OneHundredSeventyFiveSeconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "MaxDelay",
            }),
        }
    }
}
impl From<MaxDelay> for u8 {
    fn from(val: MaxDelay) -> Self {
        match val {
            MaxDelay::Disable => 0,
            MaxDelay::FiveSeconds => 1,
            MaxDelay::EightyEightSeconds => 2,
            MaxDelay::OneHundredSeventyFiveSeconds => 3,
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
    Disabled = 0,
    OneMicroSecond = 1,
    TwoMilliseconds = 2,
    FiveSeconds = 3,
}
impl core::convert::TryFrom<u8> for ComparatorDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::OneMicroSecond),
            2 => Ok(Self::TwoMilliseconds),
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
            ComparatorDeglitchTime::Disabled => 0,
            ComparatorDeglitchTime::OneMicroSecond => 1,
            ComparatorDeglitchTime::TwoMilliseconds => 2,
            ComparatorDeglitchTime::FiveSeconds => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum PsysGain {
    ZeroPoint25UAperW = 0,
    OnePoint00UAperW = 1,
}
impl core::convert::TryFrom<u8> for PsysGain {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::ZeroPoint25UAperW),
            1 => Ok(Self::OnePoint00UAperW),
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
            PsysGain::ZeroPoint25UAperW => 0,
            PsysGain::OnePoint00UAperW => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargeSenseResistorRsr {
    TenMilliOhms = 0,
    TwentyMilliOhms = 1,
}
impl core::convert::TryFrom<u8> for ChargeSenseResistorRsr {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TenMilliOhms),
            1 => Ok(Self::TwentyMilliOhms),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "ChargeSenseResistorRsr",
            }),
        }
    }
}
impl From<ChargeSenseResistorRsr> for u8 {
    fn from(val: ChargeSenseResistorRsr) -> Self {
        match val {
            ChargeSenseResistorRsr::TenMilliOhms => 0,
            ChargeSenseResistorRsr::TwentyMilliOhms => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum InputSenseResistorRac {
    TenMilliOhms = 0,
    TwentyMilliOhms = 1,
}
impl core::convert::TryFrom<u8> for InputSenseResistorRac {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TenMilliOhms),
            1 => Ok(Self::TwentyMilliOhms),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "InputSenseResistorRac",
            }),
        }
    }
}
impl From<InputSenseResistorRac> for u8 {
    fn from(val: InputSenseResistorRac) -> Self {
        match val {
            InputSenseResistorRac::TenMilliOhms => 0,
            InputSenseResistorRac::TwentyMilliOhms => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum PsysEnable {
    PbusAndPbat = 0,
    Pbus = 1,
    Reserved = 2,
    Off = 3,
}
impl core::convert::TryFrom<u8> for PsysEnable {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::PbusAndPbat),
            1 => Ok(Self::Pbus),
            2 => Ok(Self::Reserved),
            3 => Ok(Self::Off),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PsysEnable",
            }),
        }
    }
}
impl From<PsysEnable> for u8 {
    fn from(val: PsysEnable) -> Self {
        match val {
            PsysEnable::PbusAndPbat => 0,
            PsysEnable::Pbus => 1,
            PsysEnable::Reserved => 2,
            PsysEnable::Off => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum BatocVth {
    OneHunderedThirtyThreePercent = 0,
    TwoHundredPercent = 1,
}
impl core::convert::TryFrom<u8> for BatocVth {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneHunderedThirtyThreePercent),
            1 => Ok(Self::TwoHundredPercent),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "BatocVth",
            }),
        }
    }
}
impl From<BatocVth> for u8 {
    fn from(val: BatocVth) -> Self {
        match val {
            BatocVth::OneHunderedThirtyThreePercent => 0,
            BatocVth::TwoHundredPercent => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum AcocLimit {
    OneHunderedThirtyThreePercent = 0,
    TwoHundredPercent = 1,
}
impl core::convert::TryFrom<u8> for AcocLimit {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::OneHunderedThirtyThreePercent),
            1 => Ok(Self::TwoHundredPercent),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AcocLimit",
            }),
        }
    }
}
impl From<AcocLimit> for u8 {
    fn from(val: AcocLimit) -> Self {
        match val {
            AcocLimit::OneHunderedThirtyThreePercent => 0,
            AcocLimit::TwoHundredPercent => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum AcxOcp {
    TwoHundredEightyMilliVolts = 0,
    OneHundredFiftyMillivolts = 1,
}
impl core::convert::TryFrom<u8> for AcxOcp {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TwoHundredEightyMilliVolts),
            1 => Ok(Self::OneHundredFiftyMillivolts),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "AcxOcp",
            }),
        }
    }
}
impl From<AcxOcp> for u8 {
    fn from(val: AcxOcp) -> Self {
        match val {
            AcxOcp::TwoHundredEightyMilliVolts => 0,
            AcxOcp::OneHundredFiftyMillivolts => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Q2Ocp {
    TwoHundredTenMilliVolts = 0,
    OneHundredFiftyMillivolts = 1,
}
impl core::convert::TryFrom<u8> for Q2Ocp {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::TwoHundredTenMilliVolts),
            1 => Ok(Self::OneHundredFiftyMillivolts),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Q2Ocp",
            }),
        }
    }
}
impl From<Q2Ocp> for u8 {
    fn from(val: Q2Ocp) -> Self {
        match val {
            Q2Ocp::TwoHundredTenMilliVolts => 0,
            Q2Ocp::OneHundredFiftyMillivolts => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IBatPinSelect {
    IBatPinAsDischarge = 0,
    IBatPinAsCharge = 1,
}
impl core::convert::TryFrom<u8> for IBatPinSelect {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::IBatPinAsDischarge),
            1 => Ok(Self::IBatPinAsCharge),
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
            IBatPinSelect::IBatPinAsDischarge => 0,
            IBatPinSelect::IBatPinAsCharge => 1,
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
pub enum PsysOtg {
    BattDischargePowerMinusOtg = 0,
    BattDischargePower = 1,
}
impl core::convert::TryFrom<u8> for PsysOtg {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::BattDischargePowerMinusOtg),
            1 => Ok(Self::BattDischargePower),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "PsysOtg",
            }),
        }
    }
}
impl From<PsysOtg> for u8 {
    fn from(val: PsysOtg) -> Self {
        match val {
            PsysOtg::BattDischargePowerMinusOtg => 0,
            PsysOtg::BattDischargePower => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IlAvgClamp {
    SixAmps = 0,
    TenAmps = 1,
    FifteenAmps = 2,
    Disabled = 3,
}
impl core::convert::TryFrom<u8> for IlAvgClamp {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::SixAmps),
            1 => Ok(Self::TenAmps),
            2 => Ok(Self::FifteenAmps),
            3 => Ok(Self::Disabled),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IlAvgClamp",
            }),
        }
    }
}
impl From<IlAvgClamp> for u8 {
    fn from(val: IlAvgClamp) -> Self {
        match val {
            IlAvgClamp::SixAmps => 0,
            IlAvgClamp::TenAmps => 1,
            IlAvgClamp::FifteenAmps => 2,
            IlAvgClamp::Disabled => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum InomDeglitchTime {
    Short = 0,
    Long = 1,
}
impl core::convert::TryFrom<u8> for InomDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Short),
            1 => Ok(Self::Long),
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
            InomDeglitchTime::Short => 0,
            InomDeglitchTime::Long => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Threshold {
    EightyThreePercent = 0,
    NinetyOnePercent = 1,
}
impl core::convert::TryFrom<u8> for Threshold {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::EightyThreePercent),
            1 => Ok(Self::NinetyOnePercent),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "Threshold",
            }),
        }
    }
}
impl From<Threshold> for u8 {
    fn from(val: Threshold) -> Self {
        match val {
            Threshold::EightyThreePercent => 0,
            Threshold::NinetyOnePercent => 1,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IcritDeglitchTime {
    FifteenMicroSeconds = 0,
    OneHundredMicroSeconds = 1,
    FourHundredMicroSeconds = 2,
    EightHundredMicroSeconds = 3,
}
impl core::convert::TryFrom<u8> for IcritDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::FifteenMicroSeconds),
            1 => Ok(Self::OneHundredMicroSeconds),
            2 => Ok(Self::FourHundredMicroSeconds),
            3 => Ok(Self::EightHundredMicroSeconds),
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
            IcritDeglitchTime::FifteenMicroSeconds => 0,
            IcritDeglitchTime::OneHundredMicroSeconds => 1,
            IcritDeglitchTime::FourHundredMicroSeconds => 2,
            IcritDeglitchTime::EightHundredMicroSeconds => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IdchgDeglitchTime {
    SeventyEightMilliseconds = 0,
    OnePointTwoFiveSeconds = 1,
    FiveSeconds = 2,
    TwentySeconds = 3,
}
impl core::convert::TryFrom<u8> for IdchgDeglitchTime {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::SeventyEightMilliseconds),
            1 => Ok(Self::OnePointTwoFiveSeconds),
            2 => Ok(Self::FiveSeconds),
            3 => Ok(Self::TwentySeconds),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IdchgDeglitchTime",
            }),
        }
    }
}
impl From<IdchgDeglitchTime> for u8 {
    fn from(val: IdchgDeglitchTime) -> Self {
        match val {
            IdchgDeglitchTime::SeventyEightMilliseconds => 0,
            IdchgDeglitchTime::OnePointTwoFiveSeconds => 1,
            IdchgDeglitchTime::FiveSeconds => 2,
            IdchgDeglitchTime::TwentySeconds => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IdchgDeglitchTime2 {
    VeryShort = 0,
    Short = 1,
    Long = 2,
    VeryLong = 3,
}
impl core::convert::TryFrom<u8> for IdchgDeglitchTime2 {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::VeryShort),
            1 => Ok(Self::Short),
            2 => Ok(Self::Long),
            3 => Ok(Self::VeryLong),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "IdchgDeglitchTime2",
            }),
        }
    }
}
impl From<IdchgDeglitchTime2> for u8 {
    fn from(val: IdchgDeglitchTime2) -> Self {
        match val {
            IdchgDeglitchTime2::VeryShort => 0,
            IdchgDeglitchTime2::Short => 1,
            IdchgDeglitchTime2::Long => 2,
            IdchgDeglitchTime2::VeryLong => 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum DitherConfig {
    Disable = 0,
    OneX = 1,
    TwoX = 2,
    ThreeX = 3,
}
impl core::convert::TryFrom<u8> for DitherConfig {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Disable),
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
            DitherConfig::Disable => 0,
            DitherConfig::OneX => 1,
            DitherConfig::TwoX => 2,
            DitherConfig::ThreeX => 3,
        }
    }
}
