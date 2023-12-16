#include <lua.h>

int lcall(lua_State *L, int narg, int clear);
void getglobal(lua_State *L, const char *s);
