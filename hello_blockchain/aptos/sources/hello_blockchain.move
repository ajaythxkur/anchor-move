module hello_blockchain::main{
    use std::signer::address_of;
    use std::string::String;
    use std::error;

    struct Message has key{
        content: String
    }

    const ENOT_FOUND: u64 = 1;

    public entry fun create_or_update_message(account: &signer, content: String) acquires Message{
        let account_addr = address_of(account);
        if(!exists<Message>(account_addr)){
            let newMsg = Message {
                content
            };
            move_to(account, newMsg)
        }else{
            let msg = borrow_global_mut<Message>(account_addr);
            msg.content = content;
        }
    }

    #[view]
    public fun view_message(account_addr: address): String acquires Message{
        assert!(exists<Message>(account_addr), error::not_found(ENOT_FOUND));
        borrow_global<Message>(account_addr).content
    }

    // TODO: Write tests
}