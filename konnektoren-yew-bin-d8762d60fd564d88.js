import { initTonWallet, payTonWallet } from './snippets/konnektoren-yew-532cd2357bc4cd56/src/components/marketplace/wallet/ton_wallet.js';

/**
 * A hash; the 32-byte output of a hashing algorithm.
 *
 * This struct is used most often in `solana-sdk` and related crates to contain
 * a [SHA-256] hash, but may instead contain a [blake3] hash.
 *
 * [SHA-256]: https://en.wikipedia.org/wiki/SHA-2
 * [blake3]: https://github.com/BLAKE3-team/BLAKE3
 */
export class Hash {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Hash.prototype);
        obj.__wbg_ptr = ptr;
        HashFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        HashFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_hash_free(ptr, 0);
    }
    /**
     * Create a new Hash object
     *
     * * `value` - optional hash as a base58 encoded string, `Uint8Array`, `[number]`
     * @param {any} value
     */
    constructor(value) {
        const ret = wasm.hash_constructor(value);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        HashFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Checks if two `Hash`s are equal
     * @param {Hash} other
     * @returns {boolean}
     */
    equals(other) {
        _assertClass(other, Hash);
        const ret = wasm.hash_equals(this.__wbg_ptr, other.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Return the `Uint8Array` representation of the hash
     * @returns {Uint8Array}
     */
    toBytes() {
        const ret = wasm.hash_toBytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * Return the base58 string representation of the hash
     * @returns {string}
     */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.hash_toString(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Hash.prototype[Symbol.dispose] = Hash.prototype.free;

/**
 * wasm-bindgen version of the Instruction struct.
 * This duplication is required until https://github.com/rustwasm/wasm-bindgen/issues/3671
 * is fixed. This must not diverge from the regular non-wasm Instruction struct.
 */
export class Instruction {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Instruction.prototype);
        obj.__wbg_ptr = ptr;
        InstructionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InstructionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_instruction_free(ptr, 0);
    }
}
if (Symbol.dispose) Instruction.prototype[Symbol.dispose] = Instruction.prototype.free;

export class Instructions {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InstructionsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_instructions_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.instructions_constructor();
        this.__wbg_ptr = ret >>> 0;
        InstructionsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Instruction} instruction
     */
    push(instruction) {
        _assertClass(instruction, Instruction);
        var ptr0 = instruction.__destroy_into_raw();
        wasm.instructions_push(this.__wbg_ptr, ptr0);
    }
}
if (Symbol.dispose) Instructions.prototype[Symbol.dispose] = Instructions.prototype.free;

/**
 * A vanilla Ed25519 key pair
 */
export class Keypair {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Keypair.prototype);
        obj.__wbg_ptr = ptr;
        KeypairFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        KeypairFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_keypair_free(ptr, 0);
    }
    /**
     * Create a new `Keypair `
     */
    constructor() {
        const ret = wasm.keypair_constructor();
        this.__wbg_ptr = ret >>> 0;
        KeypairFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Recover a `Keypair` from a `Uint8Array`
     * @param {Uint8Array} bytes
     * @returns {Keypair}
     */
    static fromBytes(bytes) {
        const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.keypair_fromBytes(ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Keypair.__wrap(ret[0]);
    }
    /**
     * Return the `Pubkey` for this `Keypair`
     * @returns {Pubkey}
     */
    pubkey() {
        const ret = wasm.keypair_pubkey(this.__wbg_ptr);
        return Pubkey.__wrap(ret);
    }
    /**
     * Convert a `Keypair` to a `Uint8Array`
     * @returns {Uint8Array}
     */
    toBytes() {
        const ret = wasm.keypair_toBytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
}
if (Symbol.dispose) Keypair.prototype[Symbol.dispose] = Keypair.prototype.free;

/**
 * wasm-bindgen version of the Message struct.
 * This duplication is required until https://github.com/rustwasm/wasm-bindgen/issues/3671
 * is fixed. This must not diverge from the regular non-wasm Message struct.
 */
export class Message {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Message.prototype);
        obj.__wbg_ptr = ptr;
        MessageFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MessageFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_message_free(ptr, 0);
    }
    /**
     * The id of a recent ledger entry.
     * @returns {Hash}
     */
    get recent_blockhash() {
        const ret = wasm.__wbg_get_message_recent_blockhash(this.__wbg_ptr);
        return Hash.__wrap(ret);
    }
    /**
     * The id of a recent ledger entry.
     * @param {Hash} arg0
     */
    set recent_blockhash(arg0) {
        _assertClass(arg0, Hash);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_message_recent_blockhash(this.__wbg_ptr, ptr0);
    }
}
if (Symbol.dispose) Message.prototype[Symbol.dispose] = Message.prototype.free;

/**
 * The address of a [Solana account][acc].
 *
 * Some account addresses are [ed25519] public keys, with corresponding secret
 * keys that are managed off-chain. Often, though, account addresses do not
 * have corresponding secret keys &mdash; as with [_program derived
 * addresses_][pdas] &mdash; or the secret key is not relevant to the operation
 * of a program, and may have even been disposed of. As running Solana programs
 * can not safely create or manage secret keys, the full [`Keypair`] is not
 * defined in `solana-program` but in `solana-sdk`.
 *
 * [acc]: https://solana.com/docs/core/accounts
 * [ed25519]: https://ed25519.cr.yp.to/
 * [pdas]: https://solana.com/docs/core/cpi#program-derived-addresses
 * [`Keypair`]: https://docs.rs/solana-sdk/latest/solana_sdk/signer/keypair/struct.Keypair.html
 */
export class Pubkey {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Pubkey.prototype);
        obj.__wbg_ptr = ptr;
        PubkeyFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PubkeyFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pubkey_free(ptr, 0);
    }
    /**
     * Create a new Pubkey object
     *
     * * `value` - optional public key as a base58 encoded string, `Uint8Array`, `[number]`
     * @param {any} value
     */
    constructor(value) {
        const ret = wasm.pubkey_constructor(value);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        PubkeyFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Derive a program address from seeds and a program id
     * @param {any[]} seeds
     * @param {Pubkey} program_id
     * @returns {Pubkey}
     */
    static createProgramAddress(seeds, program_id) {
        const ptr0 = passArrayJsValueToWasm0(seeds, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(program_id, Pubkey);
        const ret = wasm.pubkey_createProgramAddress(ptr0, len0, program_id.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Pubkey.__wrap(ret[0]);
    }
    /**
     * Derive a Pubkey from another Pubkey, string seed, and a program id
     * @param {Pubkey} base
     * @param {string} seed
     * @param {Pubkey} owner
     * @returns {Pubkey}
     */
    static createWithSeed(base, seed, owner) {
        _assertClass(base, Pubkey);
        const ptr0 = passStringToWasm0(seed, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(owner, Pubkey);
        const ret = wasm.pubkey_createWithSeed(base.__wbg_ptr, ptr0, len0, owner.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Pubkey.__wrap(ret[0]);
    }
    /**
     * Checks if two `Pubkey`s are equal
     * @param {Pubkey} other
     * @returns {boolean}
     */
    equals(other) {
        _assertClass(other, Pubkey);
        const ret = wasm.pubkey_equals(this.__wbg_ptr, other.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Find a valid program address
     *
     * Returns:
     * * `[PubKey, number]` - the program address and bump seed
     * @param {any[]} seeds
     * @param {Pubkey} program_id
     * @returns {any}
     */
    static findProgramAddress(seeds, program_id) {
        const ptr0 = passArrayJsValueToWasm0(seeds, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(program_id, Pubkey);
        const ret = wasm.pubkey_findProgramAddress(ptr0, len0, program_id.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * Check if a `Pubkey` is on the ed25519 curve.
     * @returns {boolean}
     */
    isOnCurve() {
        const ret = wasm.pubkey_isOnCurve(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Return the `Uint8Array` representation of the public key
     * @returns {Uint8Array}
     */
    toBytes() {
        const ret = wasm.pubkey_toBytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * Return the base58 string representation of the public key
     * @returns {string}
     */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.pubkey_toString(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Pubkey.prototype[Symbol.dispose] = Pubkey.prototype.free;

export class SystemInstruction {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SystemInstructionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_systeminstruction_free(ptr, 0);
    }
    /**
     * @param {Pubkey} nonce_pubkey
     * @param {Pubkey} authorized_pubkey
     * @returns {Instruction}
     */
    static advanceNonceAccount(nonce_pubkey, authorized_pubkey) {
        _assertClass(nonce_pubkey, Pubkey);
        _assertClass(authorized_pubkey, Pubkey);
        const ret = wasm.systeminstruction_advanceNonceAccount(nonce_pubkey.__wbg_ptr, authorized_pubkey.__wbg_ptr);
        return Instruction.__wrap(ret);
    }
    /**
     * @param {Pubkey} pubkey
     * @param {bigint} space
     * @returns {Instruction}
     */
    static allocate(pubkey, space) {
        _assertClass(pubkey, Pubkey);
        const ret = wasm.systeminstruction_allocate(pubkey.__wbg_ptr, space);
        return Instruction.__wrap(ret);
    }
    /**
     * @param {Pubkey} address
     * @param {Pubkey} base
     * @param {string} seed
     * @param {bigint} space
     * @param {Pubkey} owner
     * @returns {Instruction}
     */
    static allocateWithSeed(address, base, seed, space, owner) {
        _assertClass(address, Pubkey);
        _assertClass(base, Pubkey);
        const ptr0 = passStringToWasm0(seed, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(owner, Pubkey);
        const ret = wasm.systeminstruction_allocateWithSeed(address.__wbg_ptr, base.__wbg_ptr, ptr0, len0, space, owner.__wbg_ptr);
        return Instruction.__wrap(ret);
    }
    /**
     * @param {Pubkey} pubkey
     * @param {Pubkey} owner
     * @returns {Instruction}
     */
    static assign(pubkey, owner) {
        _assertClass(pubkey, Pubkey);
        _assertClass(owner, Pubkey);
        const ret = wasm.systeminstruction_assign(pubkey.__wbg_ptr, owner.__wbg_ptr);
        return Instruction.__wrap(ret);
    }
    /**
     * @param {Pubkey} pubkey
     * @param {Pubkey} base
     * @param {string} seed
     * @param {Pubkey} owner
     * @returns {Instruction}
     */
    static assignWithSeed(pubkey, base, seed, owner) {
        _assertClass(pubkey, Pubkey);
        _assertClass(base, Pubkey);
        const ptr0 = passStringToWasm0(seed, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(owner, Pubkey);
        const ret = wasm.systeminstruction_assignWithSeed(pubkey.__wbg_ptr, base.__wbg_ptr, ptr0, len0, owner.__wbg_ptr);
        return Instruction.__wrap(ret);
    }
    /**
     * @param {Pubkey} nonce_pubkey
     * @param {Pubkey} authorized_pubkey
     * @param {Pubkey} new_authority
     * @returns {Instruction}
     */
    static authorizeNonceAccount(nonce_pubkey, authorized_pubkey, new_authority) {
        _assertClass(nonce_pubkey, Pubkey);
        _assertClass(authorized_pubkey, Pubkey);
        _assertClass(new_authority, Pubkey);
        const ret = wasm.systeminstruction_authorizeNonceAccount(nonce_pubkey.__wbg_ptr, authorized_pubkey.__wbg_ptr, new_authority.__wbg_ptr);
        return Instruction.__wrap(ret);
    }
    /**
     * @param {Pubkey} from_pubkey
     * @param {Pubkey} to_pubkey
     * @param {bigint} lamports
     * @param {bigint} space
     * @param {Pubkey} owner
     * @returns {Instruction}
     */
    static createAccount(from_pubkey, to_pubkey, lamports, space, owner) {
        _assertClass(from_pubkey, Pubkey);
        _assertClass(to_pubkey, Pubkey);
        _assertClass(owner, Pubkey);
        const ret = wasm.systeminstruction_createAccount(from_pubkey.__wbg_ptr, to_pubkey.__wbg_ptr, lamports, space, owner.__wbg_ptr);
        return Instruction.__wrap(ret);
    }
    /**
     * @param {Pubkey} from_pubkey
     * @param {Pubkey} to_pubkey
     * @param {Pubkey} base
     * @param {string} seed
     * @param {bigint} lamports
     * @param {bigint} space
     * @param {Pubkey} owner
     * @returns {Instruction}
     */
    static createAccountWithSeed(from_pubkey, to_pubkey, base, seed, lamports, space, owner) {
        _assertClass(from_pubkey, Pubkey);
        _assertClass(to_pubkey, Pubkey);
        _assertClass(base, Pubkey);
        const ptr0 = passStringToWasm0(seed, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(owner, Pubkey);
        const ret = wasm.systeminstruction_createAccountWithSeed(from_pubkey.__wbg_ptr, to_pubkey.__wbg_ptr, base.__wbg_ptr, ptr0, len0, lamports, space, owner.__wbg_ptr);
        return Instruction.__wrap(ret);
    }
    /**
     * @param {Pubkey} from_pubkey
     * @param {Pubkey} nonce_pubkey
     * @param {Pubkey} authority
     * @param {bigint} lamports
     * @returns {Array<any>}
     */
    static createNonceAccount(from_pubkey, nonce_pubkey, authority, lamports) {
        _assertClass(from_pubkey, Pubkey);
        _assertClass(nonce_pubkey, Pubkey);
        _assertClass(authority, Pubkey);
        const ret = wasm.systeminstruction_createNonceAccount(from_pubkey.__wbg_ptr, nonce_pubkey.__wbg_ptr, authority.__wbg_ptr, lamports);
        return ret;
    }
    /**
     * @param {Pubkey} from_pubkey
     * @param {Pubkey} to_pubkey
     * @param {bigint} lamports
     * @returns {Instruction}
     */
    static transfer(from_pubkey, to_pubkey, lamports) {
        _assertClass(from_pubkey, Pubkey);
        _assertClass(to_pubkey, Pubkey);
        const ret = wasm.systeminstruction_transfer(from_pubkey.__wbg_ptr, to_pubkey.__wbg_ptr, lamports);
        return Instruction.__wrap(ret);
    }
    /**
     * @param {Pubkey} from_pubkey
     * @param {Pubkey} from_base
     * @param {string} from_seed
     * @param {Pubkey} from_owner
     * @param {Pubkey} to_pubkey
     * @param {bigint} lamports
     * @returns {Instruction}
     */
    static transferWithSeed(from_pubkey, from_base, from_seed, from_owner, to_pubkey, lamports) {
        _assertClass(from_pubkey, Pubkey);
        _assertClass(from_base, Pubkey);
        const ptr0 = passStringToWasm0(from_seed, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(from_owner, Pubkey);
        _assertClass(to_pubkey, Pubkey);
        const ret = wasm.systeminstruction_transferWithSeed(from_pubkey.__wbg_ptr, from_base.__wbg_ptr, ptr0, len0, from_owner.__wbg_ptr, to_pubkey.__wbg_ptr, lamports);
        return Instruction.__wrap(ret);
    }
    /**
     * @param {Pubkey} nonce_pubkey
     * @param {Pubkey} authorized_pubkey
     * @param {Pubkey} to_pubkey
     * @param {bigint} lamports
     * @returns {Instruction}
     */
    static withdrawNonceAccount(nonce_pubkey, authorized_pubkey, to_pubkey, lamports) {
        _assertClass(nonce_pubkey, Pubkey);
        _assertClass(authorized_pubkey, Pubkey);
        _assertClass(to_pubkey, Pubkey);
        const ret = wasm.systeminstruction_withdrawNonceAccount(nonce_pubkey.__wbg_ptr, authorized_pubkey.__wbg_ptr, to_pubkey.__wbg_ptr, lamports);
        return Instruction.__wrap(ret);
    }
}
if (Symbol.dispose) SystemInstruction.prototype[Symbol.dispose] = SystemInstruction.prototype.free;

/**
 * wasm-bindgen version of the Transaction struct.
 * This duplication is required until https://github.com/rustwasm/wasm-bindgen/issues/3671
 * is fixed. This must not diverge from the regular non-wasm Transaction struct.
 */
export class Transaction {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Transaction.prototype);
        obj.__wbg_ptr = ptr;
        TransactionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TransactionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_transaction_free(ptr, 0);
    }
    /**
     * Create a new `Transaction`
     * @param {Instructions} instructions
     * @param {Pubkey | null} [payer]
     */
    constructor(instructions, payer) {
        _assertClass(instructions, Instructions);
        var ptr0 = instructions.__destroy_into_raw();
        let ptr1 = 0;
        if (!isLikeNone(payer)) {
            _assertClass(payer, Pubkey);
            ptr1 = payer.__destroy_into_raw();
        }
        const ret = wasm.transaction_constructor(ptr0, ptr1);
        this.__wbg_ptr = ret >>> 0;
        TransactionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Uint8Array} bytes
     * @returns {Transaction}
     */
    static fromBytes(bytes) {
        const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.transaction_fromBytes(ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Transaction.__wrap(ret[0]);
    }
    /**
     * @returns {boolean}
     */
    isSigned() {
        const ret = wasm.transaction_isSigned(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Return a message containing all data that should be signed.
     * @returns {Message}
     */
    message() {
        const ret = wasm.transaction_message(this.__wbg_ptr);
        return Message.__wrap(ret);
    }
    /**
     * Return the serialized message data to sign.
     * @returns {Uint8Array}
     */
    messageData() {
        const ret = wasm.transaction_messageData(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @param {Keypair} keypair
     * @param {Hash} recent_blockhash
     */
    partialSign(keypair, recent_blockhash) {
        _assertClass(keypair, Keypair);
        _assertClass(recent_blockhash, Hash);
        wasm.transaction_partialSign(this.__wbg_ptr, keypair.__wbg_ptr, recent_blockhash.__wbg_ptr);
    }
    /**
     * @returns {Uint8Array}
     */
    toBytes() {
        const ret = wasm.transaction_toBytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * Verify the transaction
     */
    verify() {
        const ret = wasm.transaction_verify(this.__wbg_ptr);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
}
if (Symbol.dispose) Transaction.prototype[Symbol.dispose] = Transaction.prototype.free;

/**
 * Initialize Javascript logging and panic handler
 */
export function solana_program_init() {
    wasm.solana_program_init();
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_55538483de6e3abe: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_Number_f257194b7002d6f9: function(arg0) {
            const ret = Number(arg0);
            return ret;
        },
        __wbg_String_8564e559799eccda: function(arg0, arg1) {
            const ret = String(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_bigint_get_as_i64_a738e80c0fe6f6a7: function(arg0, arg1) {
            const v = arg1;
            const ret = typeof(v) === 'bigint' ? v : undefined;
            getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_boolean_get_fe2a24fdfdb4064f: function(arg0) {
            const v = arg0;
            const ret = typeof(v) === 'boolean' ? v : undefined;
            return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
        },
        __wbg___wbindgen_debug_string_d89627202d0155b7: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_in_fe3eb6a509f75744: function(arg0, arg1) {
            const ret = arg0 in arg1;
            return ret;
        },
        __wbg___wbindgen_is_bigint_ca270ac12ef71091: function(arg0) {
            const ret = typeof(arg0) === 'bigint';
            return ret;
        },
        __wbg___wbindgen_is_function_2a95406423ea8626: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_null_8d90524c9e0af183: function(arg0) {
            const ret = arg0 === null;
            return ret;
        },
        __wbg___wbindgen_is_object_59a002e76b059312: function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_string_624d5244bb2bc87c: function(arg0) {
            const ret = typeof(arg0) === 'string';
            return ret;
        },
        __wbg___wbindgen_is_undefined_87a3a837f331fef5: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_jsval_eq_eedd705f9f2a4f35: function(arg0, arg1) {
            const ret = arg0 === arg1;
            return ret;
        },
        __wbg___wbindgen_jsval_loose_eq_cf851f110c48f9ba: function(arg0, arg1) {
            const ret = arg0 == arg1;
            return ret;
        },
        __wbg___wbindgen_number_get_769f3676dc20c1d7: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_string_get_f1161390414f9b59: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_5549492daedad139: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg___wbindgen_typeof_de716ff95389461d: function(arg0) {
            const ret = typeof arg0;
            return ret;
        },
        __wbg__wbg_cb_unref_fbe69bb076c16bad: function(arg0) {
            arg0._wbg_cb_unref();
        },
        __wbg_addEventListener_22cc39177d983010: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);
        }, arguments); },
        __wbg_addEventListener_ee34fcb181ae85b2: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
        }, arguments); },
        __wbg_add_42f33cc8592e3fce: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.add(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_altKey_fbd30c4040122f9a: function(arg0) {
            const ret = arg0.altKey;
            return ret;
        },
        __wbg_appendChild_79e0048b1f1b7921: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.appendChild(arg1);
            return ret;
        }, arguments); },
        __wbg_arrayBuffer_9f258d017f7107c5: function() { return handleError(function (arg0) {
            const ret = arg0.arrayBuffer();
            return ret;
        }, arguments); },
        __wbg_arrayBuffer_f64a4dc47790c6db: function(arg0) {
            const ret = arg0.arrayBuffer();
            return ret;
        },
        __wbg_body_42a78770fcb42acd: function(arg0) {
            const ret = arg0.body;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_bubbles_eb5e5d845d178a31: function(arg0) {
            const ret = arg0.bubbles;
            return ret;
        },
        __wbg_cache_key_21c521bc3dd66e9b: function(arg0) {
            const ret = arg0.__yew_subtree_cache_key;
            return isLikeNone(ret) ? 0x100000001 : (ret) >>> 0;
        },
        __wbg_call_4f2f92601568b772: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg0.call(arg1, arg2, arg3);
            return ret;
        }, arguments); },
        __wbg_call_6ae20895a60069a2: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_call_8f5d7bb070283508: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_cancelBubble_26aa93f493097d64: function(arg0) {
            const ret = arg0.cancelBubble;
            return ret;
        },
        __wbg_childNodes_0ad633dde5fc01d0: function(arg0) {
            const ret = arg0.childNodes;
            return ret;
        },
        __wbg_classList_fe2ed7188ad7c185: function(arg0) {
            const ret = arg0.classList;
            return ret;
        },
        __wbg_className_5077ca741dff2c58: function(arg0, arg1) {
            const ret = arg1.className;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_clearInterval_16e8cbbce92291d0: function(arg0) {
            const ret = clearInterval(arg0);
            return ret;
        },
        __wbg_clearTimeout_113b1cde814ec762: function(arg0) {
            const ret = clearTimeout(arg0);
            return ret;
        },
        __wbg_clientX_4136e615bf7e2a7c: function(arg0) {
            const ret = arg0.clientX;
            return ret;
        },
        __wbg_clientX_ebc7e3f31a9492bd: function(arg0) {
            const ret = arg0.clientX;
            return ret;
        },
        __wbg_clientY_26dcab94f3942474: function(arg0) {
            const ret = arg0.clientY;
            return ret;
        },
        __wbg_clientY_c39d37c3a25ed988: function(arg0) {
            const ret = arg0.clientY;
            return ret;
        },
        __wbg_clipboard_0f52d263b2afb1dc: function(arg0) {
            const ret = arg0.clipboard;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_cloneNode_d7b1e5ec312a7faf: function() { return handleError(function (arg0) {
            const ret = arg0.cloneNode();
            return ret;
        }, arguments); },
        __wbg_composedPath_058226d41776bbad: function(arg0) {
            const ret = arg0.composedPath();
            return ret;
        },
        __wbg_contains_2e46bcbe76a49ac2: function(arg0, arg1) {
            const ret = arg0.contains(arg1);
            return ret;
        },
        __wbg_createElementNS_f3714b3aee0a62f3: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            const ret = arg0.createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            return ret;
        }, arguments); },
        __wbg_createElement_a8dcfa25dbf80c51: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_createTextNode_9d7d2fed8d685efa: function(arg0, arg1, arg2) {
            const ret = arg0.createTextNode(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_crypto_38df2bab126b63dc: function(arg0) {
            const ret = arg0.crypto;
            return ret;
        },
        __wbg_crypto_b501cd47f5fc84cc: function(arg0) {
            const ret = arg0.crypto;
            return ret;
        },
        __wbg_ctrlKey_81931b7aea899650: function(arg0) {
            const ret = arg0.ctrlKey;
            return ret;
        },
        __wbg_dataTransfer_89aea0b8352d3dd3: function(arg0) {
            const ret = arg0.dataTransfer;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_debug_f0141abe14621fb0: function(arg0) {
            console.debug(arg0);
        },
        __wbg_deltaY_3a0ecaf2f6da7352: function(arg0) {
            const ret = arg0.deltaY;
            return ret;
        },
        __wbg_detail_bfefa4405dbef817: function(arg0) {
            const ret = arg0.detail;
            return ret;
        },
        __wbg_dispatchEvent_87c87cb727d84e9b: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.dispatchEvent(arg1);
            return ret;
        }, arguments); },
        __wbg_documentElement_c375315afd6a4740: function(arg0) {
            const ret = arg0.documentElement;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_document_cf512e4e2300751d: function(arg0) {
            const ret = arg0.document;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_done_19f92cb1f8738aba: function(arg0) {
            const ret = arg0.done;
            return ret;
        },
        __wbg_entries_28ed7cb892e12eff: function(arg0) {
            const ret = Object.entries(arg0);
            return ret;
        },
        __wbg_error_1a54d383f58f9652: function(arg0, arg1) {
            console.error(arg0, arg1);
        },
        __wbg_error_4c20fd6d19d38f03: function(arg0, arg1) {
            var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
            wasm.__wbindgen_free(arg0, arg1 * 4, 4);
            console.error(...v0);
        },
        __wbg_error_a6fa202b58aa1cd3: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_error_de6b86e598505246: function(arg0) {
            console.error(arg0);
        },
        __wbg_eval_19b94afea5a5ee59: function() { return handleError(function (arg0, arg1) {
            const ret = eval(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_fetch_3f39346b50886803: function(arg0, arg1) {
            const ret = arg0.fetch(arg1);
            return ret;
        },
        __wbg_fetch_a8db7782625775bc: function(arg0, arg1) {
            const ret = arg0.fetch(arg1);
            return ret;
        },
        __wbg_from_45cebbf5e49a6ac6: function(arg0) {
            const ret = Array.from(arg0);
            return ret;
        },
        __wbg_getAttribute_94c78997e59ce800: function(arg0, arg1, arg2, arg3) {
            const ret = arg1.getAttribute(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_getBoundingClientRect_a4588c60b4b781a4: function(arg0) {
            const ret = arg0.getBoundingClientRect();
            return ret;
        },
        __wbg_getElementById_7c22ba050c51a610: function(arg0, arg1, arg2) {
            const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_getItem_370efb66c7950f06: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg1.getItem(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_getRandomValues_0ece34fb6273ba4a: function(arg0) {
            const ret = arg0.getRandomValues;
            return ret;
        },
        __wbg_getRandomValues_c44a50d8cfdaebeb: function() { return handleError(function (arg0, arg1) {
            arg0.getRandomValues(arg1);
        }, arguments); },
        __wbg_getRandomValues_d49329ff89a07af1: function() { return handleError(function (arg0, arg1) {
            globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
        }, arguments); },
        __wbg_getRandomValues_fc2c42282aa7250c: function(arg0, arg1) {
            arg0.getRandomValues(arg1);
        },
        __wbg_getTime_c3af35594e283356: function(arg0) {
            const ret = arg0.getTime();
            return ret;
        },
        __wbg_getType_93511ad0271f7e7c: function(arg0, arg1, arg2) {
            const ret = arg0.getType(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_get_197d34851780fe91: function(arg0, arg1, arg2, arg3) {
            const ret = arg1.get(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_get_2bccffff9f6f041f: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg1.get(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_get_6eadbda4395687cb: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_get_94f5fc088edd3138: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_get_a50328e7325d7f9b: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_d255a35e31be91f3: function(arg0, arg1, arg2) {
            const ret = arg0[getStringFromWasm0(arg1, arg2)];
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_get_ff5f1fb220233477: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_unchecked_7c6bbabf5b0b1fbf: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_get_with_ref_key_6412cf3094599694: function(arg0, arg1) {
            const ret = arg0[arg1];
            return ret;
        },
        __wbg_has_3f87d148146a0f4e: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.has(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_hash_43bfda7f0e2a128f: function(arg0, arg1) {
            const ret = arg1.hash;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_hash_7ae903ee70c3dd41: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.hash;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_head_d83cbfe2b8863e8d: function(arg0) {
            const ret = arg0.head;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_headers_6ccffabdaab0d021: function(arg0) {
            const ret = arg0.headers;
            return ret;
        },
        __wbg_height_a030a387b18d604b: function(arg0) {
            const ret = arg0.height;
            return ret;
        },
        __wbg_history_304baab31767c8be: function() { return handleError(function (arg0) {
            const ret = arg0.history;
            return ret;
        }, arguments); },
        __wbg_host_c563222bae213767: function(arg0) {
            const ret = arg0.host;
            return ret;
        },
        __wbg_href_55909e641b55be73: function(arg0, arg1) {
            const ret = arg1.href;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_href_ec3def677e7c4414: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.href;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_href_f1519703102d4119: function(arg0, arg1) {
            const ret = arg1.href;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_info_ca2d5ad6378d3000: function(arg0) {
            console.info(arg0);
        },
        __wbg_initTonWallet_40d00ec532994308: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = initTonWallet(getStringFromWasm0(arg0, arg1), arg2, arg3);
            return ret;
        }, arguments); },
        __wbg_innerHeight_0ea00febf4940b36: function() { return handleError(function (arg0) {
            const ret = arg0.innerHeight;
            return ret;
        }, arguments); },
        __wbg_innerWidth_8b227093ee989fa1: function() { return handleError(function (arg0) {
            const ret = arg0.innerWidth;
            return ret;
        }, arguments); },
        __wbg_insertBefore_b7d9275f2d729de5: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.insertBefore(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_instanceof_ArrayBuffer_8d855993947fc3a2: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Blob_0ba6040bc29f038a: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Blob;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_ClipboardItem_bdf8f1be0210c7ed: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ClipboardItem;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Element_60d1572f735fb2ce: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Element;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Error_e14ad3dc04e9f18c: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Error;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlElement_78785ade06923935: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlMetaElement_616e11cfc31d34c1: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLMetaElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlSelectElement_cc51eb13c39fb68c: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLSelectElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Map_238410f1463c05ed: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Map;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_MouseEvent_ca7d8930748355f1: function(arg0) {
            let result;
            try {
                result = arg0 instanceof MouseEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Object_d622a5764f4f9002: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Object;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Response_fece7eabbcaca4c3: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Response;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_ShadowRoot_aa005ef99a0ca50b: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ShadowRoot;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_TouchEvent_4d936b5cff40d307: function(arg0) {
            let result;
            try {
                result = arg0 instanceof TouchEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Uint8Array_ce24d58a5f4bdcc3: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_2fa8d9c2d5b6104a: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_c99af69199cfc64f: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_WorkerGlobalScope_a4307c85f73d83c3: function(arg0) {
            let result;
            try {
                result = arg0 instanceof WorkerGlobalScope;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instruction_new: function(arg0) {
            const ret = Instruction.__wrap(arg0);
            return ret;
        },
        __wbg_isArray_867202cf8f195ed8: function(arg0) {
            const ret = Array.isArray(arg0);
            return ret;
        },
        __wbg_isSafeInteger_1dfae065cbfe1915: function(arg0) {
            const ret = Number.isSafeInteger(arg0);
            return ret;
        },
        __wbg_is_8d0dad0e07b754c5: function(arg0, arg1) {
            const ret = Object.is(arg0, arg1);
            return ret;
        },
        __wbg_iterator_54661826e186eb6a: function() {
            const ret = Symbol.iterator;
            return ret;
        },
        __wbg_keys_e84d806594765111: function(arg0) {
            const ret = Object.keys(arg0);
            return ret;
        },
        __wbg_language_5e5d314d08889096: function(arg0, arg1) {
            const ret = arg1.language;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_lastChild_69d7f331358bca60: function(arg0) {
            const ret = arg0.lastChild;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_left_3b637e6527576a90: function(arg0) {
            const ret = arg0.left;
            return ret;
        },
        __wbg_length_e6e1633fbea6cfa9: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_fae3e439140f48a4: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_listener_id_51707c1ea7d7f75c: function(arg0) {
            const ret = arg0.__yew_listener_id;
            return isLikeNone(ret) ? 0x100000001 : (ret) >>> 0;
        },
        __wbg_localStorage_6af88123156fd954: function() { return handleError(function (arg0) {
            const ret = arg0.localStorage;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_location_639bffbf5fe10651: function(arg0) {
            const ret = arg0.location;
            return ret;
        },
        __wbg_log_07cc5cf481b065a7: function(arg0, arg1) {
            console.log(arg0, arg1);
        },
        __wbg_log_5b05b85bc32255d7: function(arg0, arg1, arg2) {
            console.log(arg0, arg1, arg2);
        },
        __wbg_log_6a75b71d6316e935: function(arg0) {
            console.log(arg0);
        },
        __wbg_message_613f79663a57a145: function(arg0) {
            const ret = arg0.message;
            return ret;
        },
        __wbg_metaKey_5ad20c9c8d673f9c: function(arg0) {
            const ret = arg0.metaKey;
            return ret;
        },
        __wbg_msCrypto_56bad8adf1ceb3d9: function(arg0) {
            const ret = arg0.msCrypto;
            return ret;
        },
        __wbg_msCrypto_bd5a034af96bcba6: function(arg0) {
            const ret = arg0.msCrypto;
            return ret;
        },
        __wbg_name_3aa8b7545ecd9f40: function(arg0) {
            const ret = arg0.name;
            return ret;
        },
        __wbg_namespaceURI_2d8879ce91545d15: function(arg0, arg1) {
            const ret = arg1.namespaceURI;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_navigator_47164ffacf3edc06: function(arg0) {
            const ret = arg0.navigator;
            return ret;
        },
        __wbg_navigator_e32bda5e256bcea6: function(arg0) {
            const ret = arg0.navigator;
            return ret;
        },
        __wbg_new_0934b88171ef61b0: function() {
            const ret = new Map();
            return ret;
        },
        __wbg_new_0_e649c99e7382313f: function() {
            const ret = new Date();
            return ret;
        },
        __wbg_new_1d96678aaacca32e: function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        },
        __wbg_new_210ef5849ab6cf48: function() { return handleError(function () {
            const ret = new Headers();
            return ret;
        }, arguments); },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_new_3fcda0d9117b718b: function() { return handleError(function () {
            const ret = new URLSearchParams();
            return ret;
        }, arguments); },
        __wbg_new_4370be21fa2b2f80: function() {
            const ret = new Array();
            return ret;
        },
        __wbg_new_48e1d86cfd30c8e7: function() {
            const ret = new Object();
            return ret;
        },
        __wbg_new_4c0c9411f5d922d7: function() { return handleError(function () {
            const ret = new SpeechSynthesisUtterance();
            return ret;
        }, arguments); },
        __wbg_new_b906fdbc60e83470: function() { return handleError(function (arg0, arg1) {
            const ret = new URL(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_new_d4b1463823c6b863: function() { return handleError(function (arg0) {
            const ret = new ClipboardItem(arg0);
            return ret;
        }, arguments); },
        __wbg_new_from_slice_0bc58e36f82a1b50: function(arg0, arg1) {
            const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_new_no_args_3842c76cc1b04f11: function(arg0, arg1) {
            const ret = new Function(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_new_with_base_7d8ccfd7797e6ff4: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = new URL(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
            return ret;
        }, arguments); },
        __wbg_new_with_event_init_dict_e85082fac24c6169: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = new CustomEvent(getStringFromWasm0(arg0, arg1), arg2);
            return ret;
        }, arguments); },
        __wbg_new_with_length_0f3108b57e05ed7c: function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return ret;
        },
        __wbg_new_with_length_7a3e5757e0797ecb: function(arg0) {
            const ret = new Array(arg0 >>> 0);
            return ret;
        },
        __wbg_new_with_str_6f03957bf9e0f079: function() { return handleError(function (arg0, arg1) {
            const ret = new Request(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_new_with_str_and_init_cb3df438bf62964e: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = new Request(getStringFromWasm0(arg0, arg1), arg2);
            return ret;
        }, arguments); },
        __wbg_new_with_str_f3d00d5938d7b751: function() { return handleError(function (arg0, arg1) {
            const ret = new URLSearchParams(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_new_with_u8_array_sequence_and_options_93acf864134c05b7: function() { return handleError(function (arg0, arg1) {
            const ret = new Blob(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_nextSibling_b5ea0d24290942af: function(arg0) {
            const ret = arg0.nextSibling;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_next_55d835fe0ab5b3e7: function(arg0) {
            const ret = arg0.next;
            return ret;
        },
        __wbg_next_e34cfb9df1518d7c: function() { return handleError(function (arg0) {
            const ret = arg0.next();
            return ret;
        }, arguments); },
        __wbg_node_84ea875411254db1: function(arg0) {
            const ret = arg0.node;
            return ret;
        },
        __wbg_of_a96e15740cdace88: function(arg0) {
            const ret = Array.of(arg0);
            return ret;
        },
        __wbg_offsetX_57cce2d8ea8bc837: function(arg0) {
            const ret = arg0.offsetX;
            return ret;
        },
        __wbg_offsetY_50fccc49deb4f3d6: function(arg0) {
            const ret = arg0.offsetY;
            return ret;
        },
        __wbg_outerHTML_bf1929ba3554206f: function(arg0, arg1) {
            const ret = arg1.outerHTML;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_parentElement_90ebc389e4feec98: function(arg0) {
            const ret = arg0.parentElement;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_parentNode_762a8626d5cc08d9: function(arg0) {
            const ret = arg0.parentNode;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_pathname_5a9c966225d90655: function(arg0, arg1) {
            const ret = arg1.pathname;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_pathname_f1d35181cabf06f5: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.pathname;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_payTonWallet_543654744363661d: function() { return handleError(function (arg0, arg1, arg2) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                const ret = payTonWallet(getStringFromWasm0(arg0, arg1), BigInt.asUintN(64, arg2));
                return ret;
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        }, arguments); },
        __wbg_play_7b1e3745d94a9890: function() { return handleError(function (arg0) {
            const ret = arg0.play();
            return ret;
        }, arguments); },
        __wbg_preventDefault_a85164025505c2f9: function(arg0) {
            arg0.preventDefault();
        },
        __wbg_process_44c7a14e11e9f69e: function(arg0) {
            const ret = arg0.process;
            return ret;
        },
        __wbg_prototypesetcall_3875d54d12ef2eec: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        },
        __wbg_pubkey_new: function(arg0) {
            const ret = Pubkey.__wrap(arg0);
            return ret;
        },
        __wbg_pushState_e06769078a41fe94: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.pushState(arg1, getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        }, arguments); },
        __wbg_push_d0006a37f9fcda6d: function(arg0, arg1) {
            const ret = arg0.push(arg1);
            return ret;
        },
        __wbg_querySelector_57e670971375e069: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_queueMicrotask_8868365114fe23b5: function(arg0) {
            queueMicrotask(arg0);
        },
        __wbg_queueMicrotask_cfc5a0e62f9ebdbe: function(arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        },
        __wbg_randomFillSync_1afd9d46e5907320: function(arg0, arg1, arg2) {
            arg0.randomFillSync(getArrayU8FromWasm0(arg1, arg2));
        },
        __wbg_randomFillSync_6c25eac9869eb53c: function() { return handleError(function (arg0, arg1) {
            arg0.randomFillSync(arg1);
        }, arguments); },
        __wbg_readText_a913ba6dbb25442c: function(arg0) {
            const ret = arg0.readText();
            return ret;
        },
        __wbg_read_ffe4aa8a8fe799a1: function(arg0) {
            const ret = arg0.read();
            return ret;
        },
        __wbg_removeAttribute_a2d8bdae9c65b7af: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.removeAttribute(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_removeChild_d4e08300f4694161: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.removeChild(arg1);
            return ret;
        }, arguments); },
        __wbg_removeEventListener_05045076192ed5e7: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4 !== 0);
        }, arguments); },
        __wbg_removeItem_d18a258184a546a6: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.removeItem(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_remove_a500d12a07928a49: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.remove(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_require_6e5b8fc0b04be67c: function(arg0, arg1, arg2) {
            const ret = arg0.require(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_require_b4edbdcf3e2a1ef0: function() { return handleError(function () {
            const ret = module.require;
            return ret;
        }, arguments); },
        __wbg_resolve_d8059bc113e215bf: function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        },
        __wbg_scrollTo_b8754f025d9e0330: function(arg0, arg1) {
            window.scrollTo(arg0, arg1);
        },
        __wbg_scrollTo_db8bd74e991b9364: function(arg0, arg1) {
            arg0.scrollTo(arg1);
        },
        __wbg_scrollX_605afd8896a8cba5: function() { return handleError(function (arg0) {
            const ret = arg0.scrollX;
            return ret;
        }, arguments); },
        __wbg_scrollY_77bc9855cdc48959: function() { return handleError(function (arg0) {
            const ret = arg0.scrollY;
            return ret;
        }, arguments); },
        __wbg_search_0da984ff6207e002: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.search;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_search_a8db99ec94d5c914: function(arg0, arg1) {
            const ret = arg1.search;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_selectedIndex_a9541a4322e383a4: function(arg0) {
            const ret = arg0.selectedIndex;
            return ret;
        },
        __wbg_self_d2194f493ba20573: function() { return handleError(function () {
            const ret = self.self;
            return ret;
        }, arguments); },
        __wbg_sessionStorage_97b9f263831866d3: function() { return handleError(function (arg0) {
            const ret = arg0.sessionStorage;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_setAttribute_981e9c312a132ede: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setData_8e906f4a6ab48d5c: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setData(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setInterval_84b64f01452a246e: function() { return handleError(function (arg0, arg1) {
            const ret = setInterval(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_setItem_fd14fd8704eedc89: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setItem(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setTimeout_ef24d2fc3ad97385: function() { return handleError(function (arg0, arg1) {
            const ret = setTimeout(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_set_0b4302959e9491f2: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.set(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_set_4702dfa37c77f492: function(arg0, arg1, arg2) {
            arg0[arg1 >>> 0] = arg2;
        },
        __wbg_set_6be42768c690e380: function(arg0, arg1, arg2) {
            arg0[arg1] = arg2;
        },
        __wbg_set_8c6629931852a4a5: function(arg0, arg1, arg2) {
            const ret = arg0.set(arg1, arg2);
            return ret;
        },
        __wbg_set_991082a7a49971cf: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(arg0, arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_set_autoplay_42c17d0bfbe3372a: function(arg0, arg1) {
            arg0.autoplay = arg1 !== 0;
        },
        __wbg_set_behavior_58b1c2633b3b9627: function(arg0, arg1) {
            arg0.behavior = __wbindgen_enum_ScrollBehavior[arg1];
        },
        __wbg_set_body_e2cf9537a2f3e0be: function(arg0, arg1) {
            arg0.body = arg1;
        },
        __wbg_set_bubbles_69fcdf4f53445d58: function(arg0, arg1) {
            arg0.bubbles = arg1 !== 0;
        },
        __wbg_set_cache_key_274f2145fcb4390c: function(arg0, arg1) {
            arg0.__yew_subtree_cache_key = arg1 >>> 0;
        },
        __wbg_set_cancelable_43ec5f9ecae3c4f2: function(arg0, arg1) {
            arg0.cancelable = arg1 !== 0;
        },
        __wbg_set_capture_6b6ae72bd7165dd4: function(arg0, arg1) {
            arg0.capture = arg1 !== 0;
        },
        __wbg_set_checked_af7ab5402ef2d0c9: function(arg0, arg1) {
            arg0.checked = arg1 !== 0;
        },
        __wbg_set_className_63ca70606ca2c53a: function(arg0, arg1, arg2) {
            arg0.className = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_composed_cc3e8f91fcc15dba: function(arg0, arg1) {
            arg0.composed = arg1 !== 0;
        },
        __wbg_set_content_1aedd7ce74b89585: function(arg0, arg1, arg2) {
            arg0.content = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_detail_71267e43be64cb46: function(arg0, arg1) {
            arg0.detail = arg1;
        },
        __wbg_set_hash_75101de69065074c: function(arg0, arg1, arg2) {
            arg0.hash = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_headers_22d4b01224273a83: function(arg0, arg1) {
            arg0.headers = arg1;
        },
        __wbg_set_innerHTML_e2a0c521ebf9b8f3: function(arg0, arg1, arg2) {
            arg0.innerHTML = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_lang_aadc7bd19352ae9b: function(arg0, arg1, arg2) {
            arg0.lang = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_left_4ffc2196ec757c49: function(arg0, arg1) {
            arg0.left = arg1;
        },
        __wbg_set_listener_id_3046a6d6a2394ad9: function(arg0, arg1) {
            arg0.__yew_listener_id = arg1 >>> 0;
        },
        __wbg_set_loop_198c57d1caac290c: function(arg0, arg1) {
            arg0.loop = arg1 !== 0;
        },
        __wbg_set_method_4a4ab3faba8a018c: function(arg0, arg1, arg2) {
            arg0.method = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_nodeValue_36fce4b3dd75fc69: function(arg0, arg1, arg2) {
            arg0.nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_once_3f8e98ae18cf7525: function(arg0, arg1) {
            arg0.once = arg1 !== 0;
        },
        __wbg_set_passive_9e23184a10341acd: function(arg0, arg1) {
            arg0.passive = arg1 !== 0;
        },
        __wbg_set_search_f793f9b7d0fd753c: function(arg0, arg1, arg2) {
            arg0.search = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_src_a83b6259b44cd636: function(arg0, arg1, arg2) {
            arg0.src = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_subtree_id_fc80ace73ff247a7: function(arg0, arg1) {
            arg0.__yew_subtree_id = arg1 >>> 0;
        },
        __wbg_set_textContent_eaa1563abaeeb959: function(arg0, arg1, arg2) {
            arg0.textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_text_c6fbe70c73fd782a: function(arg0, arg1, arg2) {
            arg0.text = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_title_6ccaecd1469532e8: function(arg0, arg1, arg2) {
            arg0.title = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_top_4d15f944c4461c70: function(arg0, arg1) {
            arg0.top = arg1;
        },
        __wbg_set_type_e8ab4a5ab35ebb6f: function(arg0, arg1, arg2) {
            arg0.type = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_value_7f45d2ec472e7855: function(arg0, arg1, arg2) {
            arg0.value = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_value_c2e2ca2c9aec94f1: function(arg0, arg1, arg2) {
            arg0.value = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_volume_225f15126d546215: function(arg0, arg1) {
            arg0.volume = arg1;
        },
        __wbg_set_volume_cd075bb5d88d4a37: function(arg0, arg1) {
            arg0.volume = arg1;
        },
        __wbg_shiftKey_f53c305e9069c8d4: function(arg0) {
            const ret = arg0.shiftKey;
            return ret;
        },
        __wbg_slice_5446e93d57fd40ad: function(arg0, arg1) {
            const ret = arg1.slice();
            const ptr1 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_speak_3d81ada77b0b4b42: function(arg0, arg1) {
            arg0.speak(arg1);
        },
        __wbg_speechSynthesis_5274f566bd9a30b6: function() { return handleError(function (arg0) {
            const ret = arg0.speechSynthesis;
            return ret;
        }, arguments); },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_state_2bb480c88adcea79: function() { return handleError(function (arg0) {
            const ret = arg0.state;
            return ret;
        }, arguments); },
        __wbg_static_accessor_GLOBAL_8dfb7f5e26ebe523: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_941154efc8395cdd: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_MODULE_ef3aa2eb251158a5: function() {
            const ret = module;
            return ret;
        },
        __wbg_static_accessor_SELF_58dac9af822f561f: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_ee64f0b3d8354c0b: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_status_1ae443dc56281de7: function(arg0) {
            const ret = arg0.status;
            return ret;
        },
        __wbg_stopPropagation_73ab5ad64e9b5657: function(arg0) {
            arg0.stopPropagation();
        },
        __wbg_subarray_035d32bb24a7d55d: function(arg0, arg1, arg2) {
            const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
            return ret;
        },
        __wbg_subtree_id_6a3200546ad613b1: function(arg0) {
            const ret = arg0.__yew_subtree_id;
            return isLikeNone(ret) ? 0x100000001 : (ret) >>> 0;
        },
        __wbg_target_42ee7269f9432d0c: function(arg0) {
            const ret = arg0.target;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_target_c2d80ee4d3287cbb: function(arg0) {
            const ret = arg0.target;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_textContent_c1c48d6ac47606b1: function(arg0, arg1) {
            const ret = arg1.textContent;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_text_6d3a70da69d27961: function() { return handleError(function (arg0) {
            const ret = arg0.text();
            return ret;
        }, arguments); },
        __wbg_then_0150352e4ad20344: function(arg0, arg1, arg2) {
            const ret = arg0.then(arg1, arg2);
            return ret;
        },
        __wbg_then_5160486c67ddb98a: function(arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        },
        __wbg_then_60ee697adfaa74bb: function(arg0, arg1, arg2) {
            const ret = arg0.then(arg1, arg2);
            return ret;
        },
        __wbg_toString_553b5f6e95e3e41b: function(arg0) {
            const ret = arg0.toString();
            return ret;
        },
        __wbg_toString_9e7353a77cb415a2: function(arg0) {
            const ret = arg0.toString();
            return ret;
        },
        __wbg_top_7190df445227226f: function(arg0) {
            const ret = arg0.top;
            return ret;
        },
        __wbg_touches_2617b6114abee3ce: function(arg0) {
            const ret = arg0.touches;
            return ret;
        },
        __wbg_types_2ed32e30f00d4d4c: function(arg0) {
            const ret = arg0.types;
            return ret;
        },
        __wbg_url_bc7dc04139db6f29: function(arg0, arg1) {
            const ret = arg1.url;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_49c7d3eb375cb662: function(arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_5f8653fa438e0c94: function(arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_c3b0708d699ea640: function(arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_d5b248ce8419bd1b: function(arg0) {
            const ret = arg0.value;
            return ret;
        },
        __wbg_values_5fd004a4195b1ba3: function(arg0) {
            const ret = arg0.values();
            return ret;
        },
        __wbg_versions_276b2795b1c6a219: function(arg0) {
            const ret = arg0.versions;
            return ret;
        },
        __wbg_warn_86ef03db8cfb4dd4: function(arg0) {
            console.warn(arg0);
        },
        __wbg_width_424e80d295f0c106: function(arg0) {
            const ret = arg0.width;
            return ret;
        },
        __wbg_writeText_1d265be10631cd76: function(arg0, arg1, arg2) {
            const ret = arg0.writeText(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_write_890fc1f61990f579: function(arg0, arg1) {
            const ret = arg0.write(arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 4829, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h2b995066aa092506);
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 5485, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__hcf4cc3909d86f56e);
            return ret;
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 6925, ret: Externref, inner_ret: Some(Externref) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h30f254d639ffe1b2);
            return ret;
        },
        __wbindgen_cast_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 8239, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h1df4d716f3947a98);
            return ret;
        },
        __wbindgen_cast_0000000000000005: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("CustomEvent")], shim_idx: 5485, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__hcf4cc3909d86f56e_4);
            return ret;
        },
        __wbindgen_cast_0000000000000006: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 8167, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures________invoke__hfa8a58188c875d86);
            return ret;
        },
        __wbindgen_cast_0000000000000007: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 8193, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures________invoke__h3984ca1704778ddc);
            return ret;
        },
        __wbindgen_cast_0000000000000008: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 8214, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures________invoke__hf0907bb777ad2a2a);
            return ret;
        },
        __wbindgen_cast_0000000000000009: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [String, String], shim_idx: 4343, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h27eea9ea0466e0a7);
            return ret;
        },
        __wbindgen_cast_000000000000000a: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [String], shim_idx: 4341, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h625346aeb9614add);
            return ret;
        },
        __wbindgen_cast_000000000000000b: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 8198, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__hec937370a857d09d);
            return ret;
        },
        __wbindgen_cast_000000000000000c: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 8206, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h9f5ebace6a40dfd1);
            return ret;
        },
        __wbindgen_cast_000000000000000d: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_000000000000000e: function(arg0) {
            // Cast intrinsic for `I64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_000000000000000f: function(arg0, arg1) {
            // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
            const ret = getArrayU8FromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000010: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000011: function(arg0) {
            // Cast intrinsic for `U64 -> Externref`.
            const ret = BigInt.asUintN(64, arg0);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./konnektoren-yew-bin_bg.js": import0,
    };
}

function wasm_bindgen__convert__closures_____invoke__hec937370a857d09d(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__hec937370a857d09d(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__h9f5ebace6a40dfd1(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__h9f5ebace6a40dfd1(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__h2b995066aa092506(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h2b995066aa092506(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__hcf4cc3909d86f56e(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__hcf4cc3909d86f56e(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__hcf4cc3909d86f56e_4(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__hcf4cc3909d86f56e_4(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures________invoke__hfa8a58188c875d86(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures________invoke__hfa8a58188c875d86(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures________invoke__h3984ca1704778ddc(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures________invoke__h3984ca1704778ddc(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures________invoke__hf0907bb777ad2a2a(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures________invoke__hf0907bb777ad2a2a(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h30f254d639ffe1b2(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen__convert__closures_____invoke__h30f254d639ffe1b2(arg0, arg1, arg2);
    return ret;
}

function wasm_bindgen__convert__closures_____invoke__h1df4d716f3947a98(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen__convert__closures_____invoke__h1df4d716f3947a98(arg0, arg1, arg2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function wasm_bindgen__convert__closures_____invoke__h625346aeb9614add(arg0, arg1, arg2) {
    const ptr0 = passStringToWasm0(arg2, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.wasm_bindgen__convert__closures_____invoke__h625346aeb9614add(arg0, arg1, ptr0, len0);
}

function wasm_bindgen__convert__closures_____invoke__h27eea9ea0466e0a7(arg0, arg1, arg2, arg3) {
    const ptr0 = passStringToWasm0(arg2, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(arg3, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    wasm.wasm_bindgen__convert__closures_____invoke__h27eea9ea0466e0a7(arg0, arg1, ptr0, len0, ptr1, len1);
}


const __wbindgen_enum_ScrollBehavior = ["auto", "instant", "smooth"];
const HashFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_hash_free(ptr >>> 0, 1));
const InstructionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_instruction_free(ptr >>> 0, 1));
const InstructionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_instructions_free(ptr >>> 0, 1));
const KeypairFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_keypair_free(ptr >>> 0, 1));
const MessageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_message_free(ptr >>> 0, 1));
const PubkeyFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_pubkey_free(ptr >>> 0, 1));
const SystemInstructionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_systeminstruction_free(ptr >>> 0, 1));
const TransactionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_transaction_free(ptr >>> 0, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('konnektoren-yew-bin_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
