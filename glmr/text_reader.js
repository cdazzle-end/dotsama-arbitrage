

exports.readAddressFile = (textFile) => {
    const fs = require('fs');
    var parsedAddresses = [];
    let data;
    // let test2 = await fs.readFile(textFile, 'utf8', (err, data) => {
    //     if (err) throw err; 

    //     // console.log(data)
    //     parsedAddresses = data;
    //     // console.log(parsedAddresses);
    //     return data;
    // let parsedAddresses = [];
    // const addressIndex = data.search("0x");
    // let endIndex = addressIndex + 42;
    // // console.log(addressIndex)
    // let addr = data.slice(addressIndex, endIndex);
    // parsedAddresses.push(addr);
    // var remaining = data.slice(endIndex);
    // // console.log(addr);
    // // console.log(remaining)
    // var searchIndex = remaining.search("0x");
    // while (searchIndex >= 0) {
    //     searchIndex = remaining.search("0x");
    //     let endIndex = searchIndex + 42;
    //     let addressSlice = remaining.slice(searchIndex, endIndex);
    //     parsedAddresses.push(addressSlice);
    //     remaining = remaining.slice(endIndex);
    //     searchIndex = remaining.search("0x");
    // }
    // test = 1;
    // // console.log(parsedAddresses);
    // // console.log(parsedAddresses.length)
    // return parsedAddresses;

    // })

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
    // console.log(parsedAddresses);
    return parsedAddresses;
    // return parsedAddresses;x``
}

function main() {
    // readAddressFile("usdc.txt");
}

// main();

// export default readAddressFile;