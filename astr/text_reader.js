exports.readAddressFile = (textFile) => {
    const fs = require('fs');
    var parsedAddresses = [];
    let data;
    try {
        data = fs.readFileSync(textFile, 'utf8');
        // console.log(data);
        var remaining = data;
        var searchIndex = data.search("0x");
        while (searchIndex >= 0) {
            // searchIndex = remaining.search("0x");
            let endIndex = searchIndex + 42;
            let addressSlice = remaining.slice(searchIndex, endIndex);
            parsedAddresses.push(addressSlice);
            remaining = remaining.slice(endIndex);
            searchIndex = remaining.search("0x");
        }

    } catch (err) {
        console.log(err)
    }
    return parsedAddresses;
}

function main() {
    // readAddressFile("usdc.txt");
}