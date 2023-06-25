import * as assert from 'assert';

// You can import and use all API from the 'vscode' module
// as well as import your extension to test it
import * as vscode from 'vscode';
import * as myExtension from '../../extension';

suite('Extension Test Suite', () => {
	vscode.window.showInformationMessage('Start all tests.');

	test('Sample test', () => {
		assert.strictEqual(
			myExtension.helloFromCore(),
			"(program (function_declaration name: (identifier) parameters: (formal_parameters) body: (statement_block)))"
		);
	});
});
