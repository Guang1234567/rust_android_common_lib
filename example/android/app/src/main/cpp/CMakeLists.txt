# For more information about using CMake with Android Studio, read the
# documentation: https://d.android.com/studio/projects/add-native-code.html

# Sets the minimum version of CMake required to build the native library.

cmake_minimum_required(VERSION 3.4.1)

string(TOLOWER "${CMAKE_BUILD_TYPE}" BUILD_TYPE_TOLOWER)

if (ANDROID_ABI STREQUAL "x86_64")
    set(ALIAS_0_ANDROID_ABI "x86_64-linux-android")
    set(ALIAS_1_ANDROID_ABI "x86_64-android")
elseif (ANDROID_ABI STREQUAL "armeabi-v7a")
    set(ALIAS_0_ANDROID_ABI "armv7-linux-androideabi")
    set(ALIAS_1_ANDROID_ABI "arm-android")
else ()
    set(ALIAS_0_ANDROID_ABI "aarch64-linux-android")
    set(ALIAS_1_ANDROID_ABI "aarch64-android")
endif ()

# Creates and names a library, sets it as either STATIC
# or SHARED, and provides the relative paths to its source code.
# You can define multiple libraries, and CMake builds them for you.
# Gradle automatically packages shared libraries with your APK.

add_library( # Sets the name of the library.
        native-lib

        # Sets the library as a shared library.
        SHARED

        # Provides a relative path to your source file(s).
        native-lib.cpp)

# Searches for a specified prebuilt library and stores the path as a
# variable. Because CMake includes system libraries in the search path by
# default, you only need to specify the name of the public NDK library
# you want to add. CMake verifies that the library exists before
# completing its build.

find_library( # Sets the name of the path variable.
        log-lib

        # Specifies the name of the NDK library that
        # you want CMake to locate.
        log)

#[[
set(distribution_DIR ${CMAKE_SOURCE_DIR}/../../../../../../target/${ALIAS_0_ANDROID_ABI}/${BUILD_TYPE_TOLOWER})

add_library(
        greetings
        SHARED
        IMPORTED)

set_target_properties(
        greetings
        PROPERTIES
        IMPORTED_LOCATION ${distribution_DIR}/libgreetings.so)
]]

set(distribution_DIR ${CMAKE_SOURCE_DIR}/../jniLibs/${CMAKE_ANDROID_ARCH_ABI})

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -L${distribution_DIR}")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -L${distribution_DIR}")


# Specifies libraries CMake should link to your target library. You
# can link multiple libraries, such as libraries you define in this
# build script, prebuilt third-party libraries, or system libraries.

target_link_libraries( # Specifies the target library.
        native-lib

        # Links the target library to the log library
        # included in the NDK.
        ${log-lib}

        greetings)


#
#
#
# https://www.cnblogs.com/webor2006/p/9999063.html
# https://juejin.im/post/5b3588976fb9a00e36427131
# https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html
# https://docs.rs/jni/0.14.0/jni/
#
#