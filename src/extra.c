#include <lua.h>

int lcall(lua_State *L, int narg, int clear)
{
    int status;
    int base = lua_gettop(L) - narg; /* function index */
    lua_pushliteral(L, "_TRACEBACK");
    lua_rawget(L, LUA_GLOBALSINDEX); /* get traceback function */
    lua_insert(L, base);
    status = lua_pcall(L, narg, (clear ? 0 : LUA_MULTRET), base);
    lua_remove(L, base); /* remove traceback function */
    return status;
}

void getglobal(lua_State *L, const char *s)
{
    lua_pushstring(L, s);
    lua_gettable(L, LUA_GLOBALSINDEX);
}
