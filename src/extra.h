#include <lua.h>

int lcall(lua_State *L, int narg, int clear);
void lgetglobal(lua_State *L, const char *s);
void lregister(lua_State *L, const char* n, lua_CFunction f);
