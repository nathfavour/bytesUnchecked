const fs = require('fs');
const path = require('path');

const programs = [
    { name: 'vuln_missing_signer', path: 'programs/01-missing-signer' },
    { name: 'vuln_arithmetic_overflow', path: 'programs/02-arithmetic-overflow' },
    { name: 'vuln_pda_seed_leak', path: 'programs/03-pda-seed-leak' },
    { name: 'vuln_account_closing', path: 'programs/04-account-closing' },
    { name: 'vuln_type_confusion', path: 'programs/05-type-confusion' },
    { name: 'vuln_zero_copy_alignment', path: 'programs/06-zero-copy-alignment' },
    { name: 'vuln_hook_reentrancy', path: 'programs/07-hook-reentrancy' },
];

programs.forEach(p => {
    const idl = {
        "version": "0.1.0",
        "name": p.name,
        "instructions": [],
        "accounts": [],
        "errors": []
    };
    
    // Very basic extraction of instructions from lib.rs
    const libRs = fs.readFileSync(path.join(p.path, 'src/lib.rs'), 'utf8');
    const ixMatches = libRs.matchAll(/pub fn (\w+)\(ctx: Context<(\w+)>/g);
    for (const match of ixMatches) {
        idl.instructions.push({
            "name": match[1],
            "accounts": [], // We'd need a more complex parser for accounts
            "args": []
        });
    }

    if (!fs.existsSync('target/idl')) fs.mkdirSync('target/idl', { recursive: true });
    fs.writeFileSync(`target/idl/${p.name}.json`, JSON.stringify(idl, null, 2));
    console.log(`Generated minimal IDL for ${p.name}`);
});
