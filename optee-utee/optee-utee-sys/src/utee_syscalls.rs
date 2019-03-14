use super::*;
use libc::*;

extern "C" {
    pub fn utee_return(ret: libc::c_ulong);
    pub fn utee_log(buf: *const c_void, len: size_t);
    pub fn utee_panic(code: libc::c_ulong);
    pub fn utee_get_property(
        prop_set: c_ulong,
        index: c_ulong,
        name: *mut c_void,
        name_len: *mut uint32_t,
        buf: *mut c_void,
        blen: *mut uint32_t,
        prop_type: *mut uint32_t,
    ) -> TEE_Result;
    pub fn utee_get_property_name_to_index(
        prop_set: c_ulong,
        name: *const c_void,
        name_len: c_ulong,
        index: *mut uint32_t,
    ) -> TEE_Result;
    pub fn utee_open_ta_session(
        dest: *const TEE_UUID,
        cancel_req_to: c_ulong,
        params: *mut utee_params,
        sess: *mut uint32_t,
        ret_orig: *mut uint32_t,
    ) -> TEE_Result;
    pub fn utee_close_ta_session(sess: c_ulong) -> TEE_Result;
    pub fn utee_invoke_ta_command(
        sess: c_ulong,
        cancel_req_to: c_ulong,
        cmd_id: c_ulong,
        params: *mut utee_params,
        ret_orig: *mut uint32_t,
    ) -> TEE_Result;
    pub fn utee_check_access_rights(flags: uint32_t, buf: *const c_void, len: size_t)
        -> TEE_Result;
    pub fn utee_get_cancellation_flag(cancel: *mut uint32_t) -> TEE_Result;
    pub fn utee_unmask_cancellation(old_mask: *mut uint32_t) -> TEE_Result;
    pub fn utee_mask_cancellation(old_mask: *mut uint32_t) -> TEE_Result;
    pub fn utee_wait(timeout: c_ulong) -> TEE_Result;
    pub fn utee_get_time(cat: c_ulong, time: *mut TEE_Time) -> TEE_Result;
    pub fn utee_set_ta_time(time: *const TEE_Time) -> TEE_Result;
    pub fn utee_cryp_state_alloc(
        algo: c_ulong,
        op_mode: c_ulong,
        key1: c_ulong,
        key2: c_ulong,
        state: *mut uint32_t,
    ) -> TEE_Result;
    pub fn utee_cryp_state_copy(dst: c_ulong, src: c_ulong) -> TEE_Result;
    pub fn utee_cryp_state_free(state: c_ulong) -> TEE_Result;
    pub fn utee_hash_init(state: c_ulong, iv: *const c_void, iv_len: size_t) -> TEE_Result;
    pub fn utee_hash_update(state: c_ulong, chunk: *const c_void, chunk_size: size_t)
        -> TEE_Result;
    pub fn utee_hash_final(
        state: c_ulong,
        chunk: *const c_void,
        chunk_size: size_t,
        hash: *mut c_void,
        hash_len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_cipher_init(state: c_ulong, iv: *const c_void, iv_len: size_t) -> TEE_Result;
    pub fn utee_cipher_update(
        state: c_ulong,
        src: *const c_void,
        src_len: size_t,
        dest: *mut c_void,
        dest_len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_cipher_final(
        state: c_ulong,
        src: *const c_void,
        src_len: size_t,
        dest: *mut c_void,
        dest_len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_cryp_obj_get_info(obj: c_ulong, info: *mut TEE_ObjectInfo) -> TEE_Result;
    pub fn utee_cryp_obj_restrict_usage(obj: c_ulong, usage: c_ulong) -> TEE_Result;
    pub fn utee_cryp_obj_get_attr(
        obj: c_ulong,
        attr_id: c_ulong,
        buffer: *mut c_void,
        size: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_cryp_obj_alloc(ttype: c_ulong, max_size: c_ulong, obj: *mut uint32_t)
        -> TEE_Result;
    pub fn utee_cryp_obj_close(obj: c_ulong) -> TEE_Result;
    pub fn utee_cryp_obj_reset(obj: c_ulong) -> TEE_Result;
    pub fn utee_cryp_obj_populate(
        obj: c_ulong,
        attrs: *mut utee_attribute,
        attr_count: c_ulong,
    ) -> TEE_Result;
    pub fn utee_cryp_obj_copy(dst_obj: c_ulong, src_obj: c_ulong) -> TEE_Result;
    pub fn utee_cryp_obj_generate_key(
        obj: c_ulong,
        key_size: c_ulong,
        params: *const utee_attribute,
        param_count: c_ulong,
    ) -> TEE_Result;
    pub fn utee_cryp_derive_key(
        state: c_ulong,
        params: *const utee_attribute,
        param_count: c_ulong,
        derived_key: c_ulong,
    ) -> TEE_Result;
    pub fn utee_cryp_random_number_generate(buf: *mut c_void, blen: size_t) -> TEE_Result;
    pub fn utee_authenc_init(
        state: c_ulong,
        nonce: *const c_void,
        nonce_len: size_t,
        tag_len: size_t,
        aad_len: size_t,
        payload_len: size_t,
    ) -> TEE_Result;
    pub fn utee_authenc_update_aad(
        state: c_ulong,
        aad_data: *const c_void,
        aad_data_len: size_t,
    ) -> TEE_Result;
    pub fn utee_authenc_update_payload(
        state: c_ulong,
        src_data: *const c_void,
        src_len: size_t,
        dest_data: *mut c_void,
        dest_len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_authenc_enc_final(
        state: c_ulong,
        src_data: *const c_void,
        src_len: size_t,
        dest_data: *mut c_void,
        dest_len: *mut uint64_t,
        tag: *mut c_void,
        tag_len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_authenc_dec_final(
        state: c_ulong,
        src_data: *const c_void,
        src_len: size_t,
        dest_data: *mut c_void,
        dest_len: *mut uint64_t,
        tag: *const c_void,
        tag_len: size_t,
    ) -> TEE_Result;
    pub fn utee_asymm_operate(
        state: c_ulong,
        params: *const utee_attribute,
        num_params: c_ulong,
        src_data: *const c_void,
        src_len: size_t,
        dest_data: *mut c_void,
        dest_len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_asymm_verify(
        state: c_ulong,
        params: *const utee_attribute,
        num_params: c_ulong,
        data: *const c_void,
        data_len: size_t,
        sig: *const c_void,
        sig_len: size_t,
    ) -> TEE_Result;
    pub fn utee_storage_obj_open(
        storage_id: c_ulong,
        object_id: *const c_void,
        object_id_len: size_t,
        flags: c_ulong,
        obj: *mut uint32_t,
    ) -> TEE_Result;
    pub fn utee_storage_obj_create(
        storage_id: c_ulong,
        object_id: *const c_void,
        object_id_len: size_t,
        flags: c_ulong,
        attr: c_ulong,
        data: *const c_void,
        len: size_t,
        obj: *mut uint32_t,
    ) -> TEE_Result;
    pub fn utee_storage_obj_del(obj: c_ulong) -> TEE_Result;
    pub fn utee_storage_obj_rename(
        obj: c_ulong,
        new_obj_id: *const c_void,
        new_obj_id_len: size_t,
    ) -> TEE_Result;
    pub fn utee_storage_alloc_enum(obj_enum: *mut uint32_t) -> TEE_Result;
    pub fn utee_storage_free_enum(obj_enum: c_ulong) -> TEE_Result;
    pub fn utee_storage_reset_enum(obj_enum: c_ulong) -> TEE_Result;
    pub fn utee_storage_start_enum(obj_enum: c_ulong, storage_id: c_ulong) -> TEE_Result;
    pub fn utee_storage_next_enum(
        obj_enum: c_ulong,
        info: *mut TEE_ObjectInfo,
        obj_id: *mut c_void,
        len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_storage_obj_read(
        obj: c_ulong,
        data: *mut c_void,
        len: size_t,
        count: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_storage_obj_write(obj: c_ulong, data: *const c_void, len: size_t) -> TEE_Result;
    pub fn utee_storage_obj_trunc(obj: c_ulong, len: size_t) -> TEE_Result;
    pub fn utee_storage_obj_seek(obj: c_ulong, offset: int32_t, whence: c_ulong) -> TEE_Result;
    pub fn utee_se_service_open(seServiceHandle: *mut uint32_t) -> TEE_Result;
    pub fn utee_se_service_close(seServiceHandle: c_ulong) -> TEE_Result;
    pub fn utee_se_service_get_readers(
        seServiceHandle: c_ulong,
        r: *mut uint32_t,
        len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_se_reader_get_prop(r: c_ulong, p: *mut uint32_t) -> TEE_Result;
    pub fn utee_se_reader_get_name(
        r: c_ulong,
        name: *mut c_char,
        name_len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_se_reader_open_session(r: c_ulong, s: *mut uint32_t) -> TEE_Result;
    pub fn utee_se_reader_close_sessions(r: c_ulong) -> TEE_Result;
    pub fn utee_se_session_is_closed(s: c_ulong) -> TEE_Result;
    pub fn utee_se_session_get_atr(
        s: c_ulong,
        atr: *mut c_void,
        atr_len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_se_session_open_channel(
        s: c_ulong,
        is_logical: c_ulong,
        aid_buffer: *const c_void,
        aid_buffer_len: size_t,
        c: *mut uint32_t,
    ) -> TEE_Result;
    pub fn utee_se_session_close(s: c_ulong) -> TEE_Result;
    pub fn utee_se_channel_select_next(c: c_ulong) -> TEE_Result;
    pub fn utee_se_channel_get_select_resp(
        c: c_ulong,
        resp: *mut c_void,
        resp_len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_se_channel_transmit(
        c: c_ulong,
        cmd: *mut c_void,
        cmd_len: size_t,
        resp: *mut c_void,
        resp_len: *mut uint64_t,
    ) -> TEE_Result;
    pub fn utee_se_channel_close(c: c_ulong) -> TEE_Result;
    pub fn utee_cache_operation(va: *mut c_void, l: size_t, op: c_ulong) -> TEE_Result;
// unimplemented syscall
// pub fn utee_gprof_send(buf: *mut c_void, size: size_t, id: *mut uint32_t) -> TEE_Result;
}
