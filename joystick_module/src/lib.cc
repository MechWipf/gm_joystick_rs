#include "GarrysMod/Lua/Interface.h"
#include "GarrysMod/Lua/LuaBase.h"
#include "joystick_module/src/lib.rs.h"
#include <string.h>

using namespace GarrysMod;
using namespace GarrysMod::Lua;
using namespace rust::cxxbridge1;

const char* binaryversion = "1.2";
static struct Evd *evd;

int refresh(lua_State* state)
{
    return 0;
}

int axis(lua_State* state)
{
    int joy = state->luabase->GetNumber(1);
    int axis = state->luabase->GetNumber(2);
    auto value = evd->get_axis(joy, axis);

    state->luabase->PushNumber(value);

    return 1;
}

int button(lua_State* state)
{
    int joy = state->luabase->GetNumber(1);
    int button = state->luabase->GetNumber(2);
    auto value = evd->get_button(joy, button);

    state->luabase->PushBool(value);

    return 1;
}

int pov(lua_State* state)
{
    int joy = state->luabase->GetNumber(1);
    int pov = state->luabase->GetNumber(2);
    auto value = evd->get_pov(joy, pov);

    state->luabase->PushNumber(value);

    return 1;
}

int count(lua_State* state)
{
    int joy = state->luabase->GetNumber(1);
    int opt = state->luabase->GetNumber(2);
    float out;

    switch (opt)
    {
        case 1:
            out = evd->get_num_axes(joy);
            break;
        case 2:
            out = evd->get_num_povs(joy);
            break;
        case 3:
            out = evd->get_num_buttons(joy);
            break;
        default:
            out = evd->get_num_joysticks();
            break;
    }

    state->luabase->PushNumber(out);

    return 1;
}

int name(lua_State* state)
{
    auto joy = state->luabase->GetNumber(1);
    auto name = evd->get_joystick_name(joy);

    state->luabase->PushString(name.c_str());

    return 1;
}

int guidm(lua_State* state)
{
    auto joy = state->luabase->GetNumber(1);
    auto name = evd->get_joystick_guid(joy);

    state->luabase->PushString(name.c_str());

    return 1;
}

int restart(lua_State* state)
{
    state->luabase->PushBool(true);
    return 1;
}

int keyboardstate(lua_State* state)
{
    // Dummy stub
    if (strcmp(state->luabase->GetTypeName(1), "table")) {
        state->luabase->CreateTable();
        state->luabase->Push(-2);
    }

    return 1;
}

GMOD_MODULE_OPEN()
{
    evd = start().into_raw();

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
    stop(evd);

    return 0;
}
