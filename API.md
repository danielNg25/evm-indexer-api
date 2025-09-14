# EVM Arbitrage Bot API

This document describes the REST API endpoints available for the EVM Arbitrage Bot.

## Base URL

```
http://localhost:3000
```

## Endpoints

### Health Check

**GET** `/health`

Returns the health status of the API.

**Response:**

```json
{
    "status": "ok",
    "message": "EVM Arbitrage Bot API is running"
}
```

### Get Networks

**GET** `/networks`

Returns all available network IDs.

**Response:**

```json
{
    "networks": [1, 56, 137],
    "total_networks": 3
}
```

### Get Pools

**GET** `/networks/{network_id}/pools`

Returns all pool addresses for a specific network.

**Parameters:**

-   `network_id` (path): The network ID (e.g., 1 for Ethereum mainnet)

**Response:**

```json
{
    "network_id": 1,
    "pools": ["0x...", "0x...", "0x..."],
    "total_pools": 5
}
```

### Get Tokens

**GET** `/networks/{network_id}/tokens`

Returns all token information for a specific network.

**Parameters:**

-   `network_id` (path): The network ID (e.g., 1 for Ethereum mainnet)

**Response:**

```json
{
    "network_id": 1,
    "tokens": [
        {
            "address": "0x...",
            "symbol": "USDC",
            "name": "USD Coin",
            "decimals": 6
        }
    ],
    "total_tokens": 10
}
```

### Quote Amount In (Raw)

**POST** `/quote/amount-in/raw`

Calculates the input amount needed for a given output amount using raw token amounts.

**Request Body:**

```json
{
    "network_id": 1,
    "pool": "0x...",
    "token_in": "0x...",
    "token_out": null,
    "amount": "0x1bc16d674ec80000"
}
```

**Note:** Either `token_in` or `token_out` must be provided (not both).

**Response:**

```json
{
    "success": true,
    "result": "0x1bc16d674ec80000",
    "error": null
}
```

### Quote Amount In (Token)

**POST** `/quote/amount-in/token`

Calculates the input amount needed for a given output amount using human-readable token amounts.

**Request Body:**

```json
{
    "network_id": 1,
    "pool": "0x...",
    "token_in": "0x...",
    "token_out": null,
    "amount": "1000.0"
}
```

**Note:** Either `token_in` or `token_out` must be provided (not both).

**Response:**

```json
{
    "success": true,
    "result": "0x1bc16d674ec80000",
    "error": null
}
```

### Quote Amount Out (Raw)

**POST** `/quote/amount-out/raw`

Calculates the output amount for a given input amount using raw token amounts.

**Request Body:**

```json
{
    "network_id": 1,
    "pool": "0x...",
    "token_in": "0x...",
    "token_out": null,
    "amount": "0x1bc16d674ec80000"
}
```

**Note:** Either `token_in` or `token_out` must be provided (not both).

**Response:**

```json
{
    "success": true,
    "result": "0x1bc16d674ec80000",
    "error": null
}
```

### Quote Amount Out (Token)

**POST** `/quote/amount-out/token`

Calculates the output amount for a given input amount using human-readable token amounts.

**Request Body:**

```json
{
    "network_id": 1,
    "pool": "0x...",
    "token_in": "0x...",
    "token_out": null,
    "amount": "1000.0"
}
```

**Note:** Either `token_in` or `token_out` must be provided (not both).

**Response:**

```json
{
    "success": true,
    "result": "0x1bc16d674ec80000",
    "error": null
}
```

## Error Responses

All endpoints return appropriate HTTP status codes:

-   `200 OK` - Success
-   `400 Bad Request` - Invalid request parameters
-   `404 Not Found` - Network or resource not found
-   `500 Internal Server Error` - Server error

Error responses include an error message:

```json
{
    "success": false,
    "result": null,
    "error": "Pool not found"
}
```

## Example Usage

### Using curl

```bash
# Health check
curl http://localhost:3000/health

# Get networks
curl http://localhost:3000/networks

# Get pools for Ethereum mainnet
curl http://localhost:3000/networks/1/pools

# Quote amount in (token) - using token_in
curl -X POST http://localhost:3000/quote/amount-in/token \
  -H "Content-Type: application/json" \
  -d '{
    "network_id": 1,
    "pool": "0x10c4E72abd373295e613e3D2C2C5067d33a0e4a8",
    "token_in": "0x6055Dc6Ff1077eebe5e6D2BA1a1f53d7Ef8430dE",
    "token_out": null,
    "amount": "1000.0"
  }'

# Quote amount in (token) - using token_out
curl -X POST http://localhost:3000/quote/amount-in/token \
  -H "Content-Type: application/json" \
  -d '{
    "network_id": 1,
    "pool": "0x10c4E72abd373295e613e3D2C2C5067d33a0e4a8",
    "token_in": null,
    "token_out": "0x6055Dc6Ff1077eebe5e6D2BA1a1f53d7Ef8430dE",
    "amount": "1000.0"
  }'
```

### Using JavaScript

```javascript
// Health check
const health = await fetch('http://localhost:3000/health').then((r) =>
    r.json()
);

// Quote amount in - using token_in
const quote = await fetch('http://localhost:3000/quote/amount-in/token', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
        network_id: 1,
        pool: '0x...',
        token_in: '0x...',
        token_out: null,
        amount: '1000.0',
    }),
}).then((r) => r.json());

// Quote amount in - using token_out
const quote2 = await fetch('http://localhost:3000/quote/amount-in/token', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
        network_id: 1,
        pool: '0x...',
        token_in: null,
        token_out: '0x...',
        amount: '1000.0',
    }),
}).then((r) => r.json());
```
