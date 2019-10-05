import riemann
import fire
from riemann import networks, simple, utils
from riemann.script import examples, serialization
from riemann.encoding import addresses

debug = True
BITCOIN_TEST = "bitcoin_test"
ZCASH_TEST = "zcash_sapling_test"

bolt_redeem_script = (
    'OP_IF '
        'OP_2 {rev_pubkey} {merch_pubkey} OP_2 '   # noqa: E131
    'OP_ELSE '
        '{timeout} OP_CHECKSEQUENCEVERIFY OP_CHECKLOCKTIMEVERIFY OP_DROP '
        '{cust_pubkey} '
    'OP_ENDIF '
    'OP_CHECKSIGVERIFY OP_BOLT')

def uint32_to_bytes(number):
    num_bytes = number.to_bytes(4, 'big')
    hex_bytes = ["{0:02x}".format(n) for n in num_bytes]
    return "".join(hex_bytes)

def createMultiSigAddress(network, pubkey0, pubkey1):
    msig_script = examples.msig_two_two
    msig_scriptpubkey = msig_script.format(pk0=pubkey0, pk1=pubkey1)

    if debug: print("Multi sig script: ", msig_scriptpubkey)

    riemann.select_network(network)
    msig_address = addresses.make_p2sh_address(msig_scriptpubkey)
    # TODO: add BOLT opcode here for channel opening?
    redeem_script = msig_scriptpubkey

    return msig_address, redeem_script


class PyScript(object):
    """Compute various functions for Bitcoin Script/Interpreter"""
    def __init__(self):
        pass

    def _get_network(self, coin):
        network = ""
        if coin == "bitcoin":
            network = BITCOIN_TEST
        elif coin == "zcash":
            network = ZCASH_TEST
        return network

    def multisig(self, coin, pk0, pk1):
        network = self._get_network(coin)
        msig_address, redeem_script = createMultiSigAddress(network, pk0, pk1)

        print("P2SH address: '{}'".format(msig_address))
        print("redeem script: '{}'".format(redeem_script))
        return

    def pay_pubkey(self, coin, pk_bytes):
        network = self._get_network(coin)
        riemann.select_network(network)
        pk_address = addresses.make_p2pkh_address(pk_bytes.encode('utf8'))
        print("P2PKH address: ", pk_address)
        return

if __name__ == "__main__":
    fire.Fire(PyScript)
