'use strict';

var core = require('@tauri-apps/api/core');

class Rusqlite {
    constructor(name) {
        this.name = name;
    }
    static async openInMemory(name) {
        return await core.invoke('plugin:rusqlite|open_in_memory', { name: name }).then(() => new Rusqlite(name));
    }
    static async openInPath(path) {
        return await core.invoke('plugin:rusqlite|open_in_path', { path: path }).then(() => new Rusqlite(path));
    }
    async migration(migrations) {
        return await core.invoke('plugin:rusqlite|migration', { name: this.name, migrations });
    }
    async update(sql, parameters) {
        return await core.invoke('plugin:rusqlite|update', { name: this.name, sql, parameters });
    }
    async select(sql, parameters) {
        return await core.invoke('plugin:rusqlite|select', { name: this.name, sql, parameters });
    }
    async batch(sql) {
        return await core.invoke('plugin:rusqlite|batch', { name: this.name, sql });
    }
    async close() {
        return await core.invoke('plugin:rusqlite|close', { name: this.name });
    }
}

module.exports = Rusqlite;
