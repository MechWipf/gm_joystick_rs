#include "GarrysMod/Lua/Interface.h"
#include "GarrysMod/Lua/LuaBase.h"
#include "joystick_module/src/lib.rs.h"

using namespace GarrysMod;
using namespace GarrysMod::Lua;

const char* binaryversion = "1.2";

int refresh(lua_State* state)
{
    return 0;
}

int axis(lua_State* state)
{
    return 0;
}

int button(lua_State* state)
{
    return 0;
}

int pov(lua_State* state)
{
    return 0;
}

int count(lua_State* state)
{
    return 0;
}

int name(lua_State* state)
{
    return 0;
}

int guidm(lua_State* state)
{
    return 0;
}

int restart(lua_State* state)
{
    state->luabase->PushBool(true);
    return 1;
}

int keyboardstate(lua_State* state)
{
    return 0;
}

GMOD_MODULE_OPEN()
{
    auto evd = start();

    LUA->PushSpecial(GarrysMod::Lua::SPECIAL_GLOB);
    LUA->CreateTable();
    LUA->PushCFunction(refresh); LUA->SetField(-2, "refresh");
    LUA->PushCFunction(axis); LUA->SetField(-2, "axis");
    LUA->PushCFunction(button); LUA->SetField(-2, "button");
    LUA->PushCFunction(pov); LUA->SetField(-2, "pov");
    LUA->PushCFunction(count); LUA->SetField(-2, "count");
    LUA->PushCFunction(name); LUA->SetField(-2, "name");
    LUA->PushCFunction(guidm); LUA->SetField(-2, "guidm");
    LUA->PushCFunction(restart); LUA->SetField(-2, "restart");
    LUA->PushString(binaryversion); LUA->SetField(-2, "binaryversion");
    LUA->PushCFunction(keyboardstate); LUA->SetField(-2, "keyboardstate");
    LUA->SetField(-2, "joystick");
    LUA->Pop(); 

    return 0;
}

GMOD_MODULE_CLOSE()
{
    return 0;
}
