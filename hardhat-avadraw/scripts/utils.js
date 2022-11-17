const fs = require('fs')
const path = require('path')
const hre = require("hardhat");

function getSavedContractAddresses() {
    let json
    try {
        json = fs.readFileSync(path.join(__dirname, '../contract-addresses.json'))
    } catch (err) {
        json = '{}'
    }
    const addrs = JSON.parse(json)
    return addrs
}
function getSavedContractABI() {
    let json
    try {
        json = fs.readFileSync(path.join(__dirname, `../artifacts/contracts/Board.sol/Board.json`))
    } catch (err) {
        json = '{}'
    }
    return JSON.parse(json)
}

function getSavedProxyABI() {
    let json
    try {
        json = fs.readFileSync(path.join(__dirname, `../proxy-abis.json`))
    } catch (err) {
        json = '{}'
    }
    return JSON.parse(json)
}

function saveContractAddress(contract, address) {
    let network = hre.network.name;
    const addrs = getSavedContractAddresses()
    addrs[network] = addrs[network] || {}
    addrs[network][contract] = address
    addrs[network]["url"] = hre.network.config.url;
    fs.writeFileSync(path.join(__dirname, '../contract-addresses.json'), JSON.stringify(addrs, null, '    '))
}

module.exports = {
    getSavedContractAddresses,
    saveContractAddress,
    getSavedContractABI,
    getSavedProxyABI
}
