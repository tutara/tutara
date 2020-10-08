export class TokenSet {
	constructor() {
		this._tokens = [];
	}

	append(token) {
		this._tokens.push(token);
	}

	get tokens() {
		return this._tokens;
	}
}
