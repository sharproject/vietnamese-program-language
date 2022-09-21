#include "variable.c"

bool endWith(const char *string, const char *suffix);
bool CompareString(const char *s1, const char *s2);
bool startsWith(const char *pre, const char *str);

void process(char *line, struct Config *configs)
{
	line = trimString(line);
	struct Config *config = findConfig(line, configs);

	if (config != NULL)
	{
		if (CompareString(config->value, "print"))
		{
			char *PrintStr = replaceWord(line, concat(config->name, ":"), "");
			PrintStr = trimString(PrintStr);
			if (CompareString(PrintStr, ""))
			{
				printf("\n");
				return;
			}
			if (startsWith("\"", PrintStr) && endWith(PrintStr, "\""))
			{
				PrintStr = sliceChar(PrintStr, 1, strlen(PrintStr) - 1);
			}
			else if (!(isNumber(PrintStr)))
			{
				struct Map *variable = getVariable(PrintStr);
				if (variable == NULL)
				{
					printf("variable [%s] is not defined \n", PrintStr);
					exit(1);
				}
				PrintStr = variable->value;
			}
			printf("%s\n", PrintStr);
		}
		if (CompareString(config->value, "variable"))
		{
			char *data = replaceWord(line, config->name, "");
			data = trimString(data);
			char *name = sliceChar(data, 0, indexOf(data, '='));
			char *value = sliceChar(data, indexOf(data, '=') + 1, strlen(data));
			name = trimString(name);
			if (name == "")
			{
				return;
			}
			value = trimString(value);
			if (value == "")
			{
				return;
			}
			struct Config *VariableNameIsKeyWord = findConfigWithName(name, configs);
			if (VariableNameIsKeyWord != NULL)
			{
				printf("can not declare variable %s because that is the keyword \n", name);
				exit(1);
			}
			if (startsWith("\"", value) && endWith(value, "\""))
			{
				value = sliceChar(value, 1, strlen(value) - 1);
			}
			newVariable(name, value);
		}
	}
	char createVarSymbol[] = ":=";
	char setVarValue[] = "=";
	if (strstr(line, createVarSymbol) != NULL)
	{
		char *found = strstr(line, createVarSymbol);
		int index = found - line;

		char *value = sliceChar(line, index + strlen(createVarSymbol), strlen(line));
		char *name = sliceChar(line, 0, index);
		name = trimString(name);
		if (name == "")
			return;
		value = trimString(value);
		if (value == "")
			return;
		struct Config *VariableNameIsKeyWord = findConfigWithName(name, configs);
		if (VariableNameIsKeyWord != NULL)
		{
			printf("can not declare variable %s because that is the keyword \n", name);
			exit(1);
		}
		if (startsWith("\"", value) && endWith(value, "\""))
		{
			value = sliceChar(value, 1, strlen(value) - 1);
		}
		newVariable(name, value);
	}
	else if (strstr(line, setVarValue) != NULL && config == NULL)
	{
		char *found = strstr(line, setVarValue);
		int index = found - line;
		char *value = sliceChar(line, index + strlen(setVarValue), strlen(line));
		char *name = sliceChar(line, 0, index);
		struct Map *variable = getVariable(name);
		if (variable == NULL)
		{
			printf("variable %s is not defined \n", name);
			exit(1);
		}
		setVariable(name, value);
	}
}