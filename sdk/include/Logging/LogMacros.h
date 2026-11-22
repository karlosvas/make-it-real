// UE_LOG y categorías de log. La salida aparece en la consola "Output Log" de Make It Real.
#pragma once
#include "Mir/Core.h"

namespace ELogVerbosity
{
	enum Type : uint8
	{
		NoLogging = 0,
		Fatal,
		Error,
		Warning,
		Display,
		Log,
		Verbose,
		VeryVerbose,
		All = VeryVerbose,
	};
}

struct FLogCategoryBase
{
	const char* MirName;
};

#define DECLARE_LOG_CATEGORY_EXTERN(CategoryName, DefaultVerbosity, CompileTimeVerbosity) extern FLogCategoryBase CategoryName;
#define DEFINE_LOG_CATEGORY(CategoryName) FLogCategoryBase CategoryName{#CategoryName};
#define DEFINE_LOG_CATEGORY_STATIC(CategoryName, DefaultVerbosity, CompileTimeVerbosity) static FLogCategoryBase CategoryName{#CategoryName};

extern FLogCategoryBase LogTemp;

#define UE_LOG(CategoryName, Verbosity, Format, ...) \
	::Mir::Log((CategoryName).MirName, (int32)ELogVerbosity::Verbosity, Format, ##__VA_ARGS__)
