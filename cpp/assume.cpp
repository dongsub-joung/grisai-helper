void check_serial_number(std::string& sn){
    uint32_t machine_hash = get_machine_hash();
    uint32_t sn_hash = calc_hash(sn);

    // we want to modify the result of this comparison.
    if (sn_hash == machine_hash) {
        // success
    }
    // fail
}
