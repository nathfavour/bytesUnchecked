const fs = require('fs');
const path = require('path');

const programs = [
    { name: 'vuln_missing_signer', path: 'programs/01-missing-signer', address: '2ZhmUXbTTZaTm9gfvV2PvBjBARQHqEaEQTsKa8QT1FV9' },
    { name: 'vuln_arithmetic_overflow', path: 'programs/02-arithmetic-overflow', address: '8i1bT2wgk6BRtE7Mog5Aj7owv8XqHhUkFfh3ZJWBcutJ' },
    { name: 'vuln_pda_seed_leak', path: 'programs/03-pda-seed-leak', address: 'D7mu2Eyx2dyCoMZMgT55zRzRTjcuTNY9DpTWbbwEs5vo' },
    { name: 'vuln_account_closing', path: 'programs/04-account-closing', address: 'Fn4k81tW7pfTdjtVcEXEGnwmvSBCmaSi9EjZPgcD8M39' },
    { name: 'vuln_type_confusion', path: 'programs/05-type-confusion', address: '8RR34N7BHCmaD1FDEuB9R2XwnHsEtXy766vCdki4KnFN' },
    { name: 'vuln_zero_copy_alignment', path: 'programs/06-zero-copy-alignment', address: 'ELR9aHdp6CkR1zwZ6sgHCDqcfaE44uqbrJsn5Bw7fmVN' },
    { name: 'vuln_hook_reentrancy', path: 'programs/07-hook-reentrancy', address: 'BjVwjTm3TzEYN9uRZx78HDQ4g1kWSCWURMeJKuzfr8vY' },
];

const crypto = require('crypto');

function getDiscriminator(name) {
    const hash = crypto.createHash('sha256').update(`global:${name}`).digest();
    return hash.slice(0, 8);
}

programs.forEach(p => {
    const idl = {
        "address": p.address,
        "metadata": {
            "name": p.name,
            "version": "0.1.0",
            "spec": "0.1.0"
        },
        "instructions": [],
        "accounts": [],
        "errors": [],
        "types": []
    };
    
    const libRs = fs.readFileSync(path.join(p.path, 'src/lib.rs'), 'utf8');
    const ixMatches = libRs.matchAll(/pub fn (\w+)\(ctx: Context<(\w+)>/g);
    for (const match of ixMatches) {
        idl.instructions.push({
            "name": match[1],
            "discriminator": Array.from(getDiscriminator(match[1])),
            "accounts": [], 
            "args": []
        });
    }

    if (!fs.existsSync('target/idl')) fs.mkdirSync('target/idl', { recursive: true });
    fs.writeFileSync(`target/idl/${p.name}.json`, JSON.stringify(idl, null, 2));
    console.log(`Generated minimal IDL for ${p.name}`);

    // Generate minimal TypeScript types
    const typeName = p.name.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join('');
    const tsContent = `export type ${typeName} = ${JSON.stringify(idl, null, 2)};`;
    if (!fs.existsSync('target/types')) fs.mkdirSync('target/types', { recursive: true });
    fs.writeFileSync(`target/types/${p.name}.ts`, tsContent);
    console.log(`Generated minimal types for ${p.name}`);
});
