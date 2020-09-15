
sudo apt-get install -y build-essential cmake pkg-config
sudo apt-get install -y libjpeg-dev libtiff5-dev libjasper-dev libpng12-dev
sudo apt-get install -y libavcodec-dev libavformat-dev libswscale-dev libv4l-dev
sudo apt-get install -y libxvidcore-dev libx264-dev
sudo apt-get install -y libatlas-base-dev gfortran

sudo apt-get install -y python3-dev
sudo apt-get install -y python3-numpy

mkdir opencv_all && cd opencv_all \
    && wget -O opencv.tar.gz https://github.com/opencv/opencv/archive/4.1.0.tar.gz \
    && tar xf opencv.tar.gz \
    && wget -O opencv_contrib.tar.gz https://github.com/opencv/opencv_contrib/archive/4.1.0.tar.gz \
    && tar xf opencv_contrib.tar.gz \
    && rm *.tar.gz

cd opencv-4.1.0 && mkdir build && cd build

cmake   -D CMAKE_BUILD_TYPE=RELEASE \
        -D CMAKE_INSTALL_PREFIX=/usr/local \
        -D OPENCV_EXTRA_MODULES_PATH=~/opencv_all/opencv_contrib-4.1.0/modules \
        -D OPENCV_ENABLE_NONFREE=ON \
        -D ENABLE_NEON=ON \
        -D ENABLE_VFPV3=ON \
        -D BUILD_TESTS=OFF \
        -D BUILD_DOCS=OFF \
        -D INSTALL_PYTHON_EXAMPLES=OFF \
        -D BUILD_EXAMPLES=OFF \
        -D PYTHON3_INCLUDE_PATH=/usr/include/python3.7m \
        -D PYTHON3_LIBRARIES=/usr/lib/arm-linux-gnueabihf/libpython3.7m.so \
        -D PYTHON3_NUMPY_INCLUDE_DIRS=/usr/lib/python3/dist-packages/numpy/core/include \
        -D BUILD_OPENCV_PYTHON2=OFF \
        -D BUILD_OPENCV_PYTHON3=ON \
        -D OPENCV_GENERATE_PKGCONFIG=ON ..

make -j4

sudo make install && sudo ldconfig
