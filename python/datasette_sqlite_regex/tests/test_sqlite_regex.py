from datasette.app import Datasette
import pytest


@pytest.mark.asyncio
async def test_plugin_is_installed():
    datasette = Datasette(memory=True)
    response = await datasette.client.get("/-/plugins.json")
    assert response.status_code == 200
    installed_plugins = {p["name"] for p in response.json()}
    assert "datasette-sqlite-regex" in installed_plugins

@pytest.mark.asyncio
async def test_sqlite_regex_functions():
    datasette = Datasette(memory=True)
    response = await datasette.client.get("/_memory.json?sql=select+regex_version(),regex()")
    assert response.status_code == 200
    regex_version, regex = response.json()["rows"][0]
    assert regex_version[0] == "v"
    assert len(regex) == 26