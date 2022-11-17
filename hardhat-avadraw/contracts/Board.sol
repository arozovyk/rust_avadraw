// SPDX-License-Identifier: UNLICENSED

pragma solidity 0.8.16;
pragma experimental ABIEncoderV2;

contract Board {
    uint256 constant size = 2000;
    uint256 constant tileSize = 20;
    uint256 constant tilesNb = size / tileSize;
    uint256 constant initialPrice = 100 gwei;
    uint8 constant minimalRate = 130;
    /* State */
    address private owner;
    mapping(uint8 => mapping(uint8 => Tile)) public tiles;
    mapping(address => uint256) public pendingWithdrawals;

    /* Tile information */
    struct Tile {
        uint256 price;
        address owner;
    }

    struct Zone {
        uint8 x;
        uint8 y;
        uint8 dx;
        uint8 dy;
    }

    struct BuyInfo {
        bool buy_self;
        uint16 buy_rate;
    }

    struct LocalPriceInfo {
        uint256 initPrice;
        uint256 price;
        uint256 priceCpt;
    }

    struct LocalUniformBuyInfo {
        uint256 max;
        uint16 cpt;
    }

    /* Events */
    event Buy(
        Zone zone,
        uint256 price,
        string url,
        bool buy_self,
        address owner
    );

    event UniformalBuy(
        Zone zone,
        uint256 price,
        string url,
        bool buy_self,
        address owner
    );
    event DrawPixel(
        uint16 x,
        uint16 y,
        uint8[4] color,
        bool overlay,
        address emiter
    );
    event DrawText(
        uint16 x,
        uint16 y,
        string text,
        uint16 text_height,
        uint8[4] text_color,
        uint8[4] background_color,
        string font_family,
        string font_name,
        bool overlay,
        address emiter
    );
    event DrawImage(
        uint16 x,
        uint16 y,
        string url,
        bool overlay,
        address emiter
    );

    event ChangeUrl(Zone zone, string url, address owner);

    constructor() {
        owner = msg.sender;
    }

    /* Count price of selected zone */
    function countPrice(
        uint8 x,
        uint8 y,
        uint8 width,
        uint8 height,
        bool buy_self
    ) public view returns (uint256 count) {
        for (uint8 i = x; i < x + width; i++) {
            for (uint8 j = y; j < y + height; j++) {
                Tile memory tile = tiles[i][j];
                if (
                    buy_self ||
                    (tile.owner != msg.sender &&
                        (tile.owner != address(0) || msg.sender != owner))
                ) {
                    if (tile.price == 0) {
                        count += initialPrice;
                    } else {
                        count += tile.price;
                    }
                }
            }
        }
    }

    function countPriceWithRate(
        uint8 x,
        uint8 y,
        uint8 width,
        uint8 height,
        bool buy_self,
        uint8 rate
    ) public view returns (uint256 count) {
        count = (countPrice(x, y, width, height, buy_self) * rate) / 100;
    }

    function test(uint256 dede, uint256 dodo) public view returns (uint256) {
        return dede * dodo ;
    }

    function getZoneInfo(
        uint8 x,
        uint8 y,
        uint8 width,
        uint8 height
    ) public view returns (Tile[][] memory) {
        Tile[][] memory result = new Tile[][](width);
        uint8 k = 0;
        uint8 l;
        for (uint8 i = x; i < x + width; i++) {
            l = 0;
            result[k] = new Tile[](height);
            for (uint8 j = y; j < y + height; j++) {
                Tile memory tile = tiles[i][j];
                result[k][l++] = tile;
            }
            k++;
        }
        return result;
    }

    /* Search for maximal tile price and count number of considered tiles */
    function maxPriceAndCpt(
        uint8 x,
        uint8 y,
        uint8 width,
        uint8 height,
        bool buy_self
    ) internal view returns (LocalUniformBuyInfo memory) {
        LocalUniformBuyInfo memory bi = LocalUniformBuyInfo(initialPrice, 0);
        for (uint8 i = x; i < x + width; i++) {
            for (uint8 j = y; j < y + height; j++) {
                Tile memory tile = tiles[i][j];
                if (
                    buy_self ||
                    (tile.owner != msg.sender &&
                        (tile.owner != address(0) || msg.sender != owner))
                ) {
                    bi.cpt++;
                    if (bi.max < tile.price) {
                        bi.max = tile.price;
                    }
                }
            }
        }
        return bi;
    }

    /* Modifiers */
    modifier correctZone(
        uint8 x,
        uint8 y,
        uint8 width,
        uint8 height
    ) {
        require(
            0 <= x && x + width < tilesNb && width > 0,
            "Wrong zone provided"
        );
        require(
            0 <= y && y + height < tilesNb && height > 0,
            "Wrong zone provided"
        );
        _;
    }

    modifier correctPixel(uint16 x, uint16 y) {
        require(0 <= x && x < size, "Wrong pixel provided");
        require(0 <= y && y < size, "Wrong pixel provided");
        _;
    }

    modifier onlyBy(address account) {
        require(msg.sender == account, "Sender not authorized.");
        _;
    }

    /* Change owner address */
    function changeOwner(address newOwner) public onlyBy(owner) {
        owner = newOwner;
    }

    /* Withdrawing funds from contract */
    function withdraw() public {
        uint256 amount = pendingWithdrawals[msg.sender];
        pendingWithdrawals[msg.sender] = 0;
        payable(msg.sender).transfer(amount);
    }

    /* Buy tiles */
    function buy(
        Zone calldata zone,
        BuyInfo calldata buy_info,
        string calldata ulr,
        address referer
    ) public payable correctZone(zone.x, zone.y, zone.dx, zone.dy) {
        LocalPriceInfo memory p;
        p.initPrice = countPrice(
            zone.x,
            zone.y,
            zone.dx,
            zone.dy,
            buy_info.buy_self
        );
        p.price = msg.value;
        p.priceCpt = msg.value;
        require(
            p.price >= (p.initPrice * buy_info.buy_rate) / 100,
            "Provided price is unsufficient"
        );
        require(buy_info.buy_rate >= minimalRate, "Provided rate is incorrect");
        for (uint8 i = zone.x; i < zone.x + zone.dx; i++) {
            for (uint8 j = zone.y; j < zone.y + zone.dy; j++) {
                Tile memory tile = tiles[i][j];
                if (
                    buy_info.buy_self ||
                    (tile.owner != msg.sender &&
                        (owner != msg.sender || tile.owner != address(0)))
                ) {
                    uint256 oldTilePrice;
                    if (tile.owner == address(0)) {
                        oldTilePrice = initialPrice;
                    } else {
                        oldTilePrice = tile.price;
                    }
                    uint256 newTilePrice = (oldTilePrice * buy_info.buy_rate) /
                        100;
                    uint256 part = (newTilePrice - oldTilePrice) / 3;
                    p.priceCpt -= part * 3 + oldTilePrice;
                    tiles[i][j].price = newTilePrice;
                    tiles[i][j].owner = msg.sender;
                    if (tile.owner == address(0)) {
                        pendingWithdrawals[owner] += oldTilePrice + part;
                    } else {
                        pendingWithdrawals[tile.owner] += oldTilePrice + part;
                    }
                    pendingWithdrawals[referer] += part;
                    pendingWithdrawals[owner] += part;
                }
            }
        }
        if (p.priceCpt > 0) {
            pendingWithdrawals[owner] += p.priceCpt;
        }
        emit Buy(zone, p.price, ulr, buy_info.buy_self, msg.sender);
    }

    /* Buy tiles uniformally */
    function buyUniform(
        Zone calldata zone,
        BuyInfo calldata buy_info,
        string calldata ulr,
        address referer,
        uint256 tilePrice
    ) public payable correctZone(zone.x, zone.y, zone.dx, zone.dy) {
        LocalPriceInfo memory p;
        LocalUniformBuyInfo memory bi = maxPriceAndCpt(
            zone.x,
            zone.y,
            zone.dx,
            zone.dy,
            buy_info.buy_self
        );
        p.price = msg.value;
        p.priceCpt = msg.value;
        require(
            tilePrice >= (bi.max * minimalRate) / 100,
            "Provided price by tile is unsufficient"
        );
        require(
            p.price >= tilePrice * bi.cpt,
            "Provided price is unsufficient"
        );
        for (uint8 i = zone.x; i < zone.x + zone.dx; i++) {
            for (uint8 j = zone.y; j < zone.y + zone.dy; j++) {
                Tile memory tile = tiles[i][j];
                if (
                    buy_info.buy_self ||
                    (tile.owner != msg.sender &&
                        (owner != msg.sender || tile.owner != address(0)))
                ) {
                    uint256 oldTilePrice;
                    if (tile.owner == address(0)) {
                        oldTilePrice = initialPrice;
                    } else {
                        oldTilePrice = tile.price;
                    }
                    uint256 part = (tilePrice - oldTilePrice) / 3;
                    p.priceCpt -= part * 3 + oldTilePrice;
                    tiles[i][j].price = tilePrice;
                    tiles[i][j].owner = msg.sender;
                    if (tile.owner == address(0)) {
                        pendingWithdrawals[owner] += oldTilePrice + part;
                    } else {
                        pendingWithdrawals[tile.owner] += oldTilePrice + part;
                    }
                    pendingWithdrawals[referer] += part;
                    pendingWithdrawals[owner] += part;
                }
            }
        }
        if (p.priceCpt > 0) {
            pendingWithdrawals[msg.sender] += p.priceCpt;
        }
        emit UniformalBuy(zone, tilePrice, ulr, buy_info.buy_self, msg.sender);
    }

    /* Draw pixel on the board */
    function drawPixel(
        uint16 x,
        uint16 y,
        uint8[4] calldata color,
        bool overlay
    ) public correctPixel(x, y) {
        emit DrawPixel(x, y, color, overlay, msg.sender);
    }

    /* Draw image on the board */
    function drawImage(
        uint16 x,
        uint16 y,
        string calldata url,
        bool overlay
    ) public correctPixel(x, y) {
        emit DrawImage(x, y, url, overlay, msg.sender);
    }

    /* Draw text on the board */
    function drawText(
        uint16 x,
        uint16 y,
        string memory text,
        uint16 text_height,
        uint8[4] calldata text_color,
        uint8[4] calldata background_color,
        string memory font_family,
        string memory font_name,
        bool overlay
    ) public correctPixel(x, y) {
        emit DrawText(
            x,
            y,
            text,
            text_height,
            text_color,
            background_color,
            font_family,
            font_name,
            overlay,
            msg.sender
        );
    }

    function changeUrl(Zone calldata zone, string calldata url)
        public
        correctZone(zone.x, zone.y, zone.dx, zone.dy)
    {
        emit ChangeUrl(zone, url, msg.sender);
    }
}
