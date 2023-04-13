# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Modifier::ModifierValidator do
  let(:modifier) { build(:modifier) }
  let(:modifier_validator) { described_class.new(modifier) }

  describe '#border=' do
    subject { modifier.borders }

    {
      ::CSVPlusPlus::Modifier::BorderSide::All => 'ALL',
      ::CSVPlusPlus::Modifier::BorderSide::Top => 'top',
      ::CSVPlusPlus::Modifier::BorderSide::Bottom => 'bottoM',
      ::CSVPlusPlus::Modifier::BorderSide::Right => 'right',
      ::CSVPlusPlus::Modifier::BorderSide::Left => 'LEFT'
    }.each do |expected_value, value|
      context value do
        before { modifier_validator.border = value }

        it { is_expected.to(include(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.border = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#bordercolor=' do
    subject { modifier.bordercolor }

    context '#FF00AA' do
      before { modifier_validator.bordercolor = '#FF00AA' }

      it { is_expected.to(be_a(::CSVPlusPlus::Color)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.bordercolor = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#borderstyle=' do
    subject { modifier.borderstyle }

    {
      ::CSVPlusPlus::Modifier::BorderStyle::Dashed => 'DASHED',
      ::CSVPlusPlus::Modifier::BorderStyle::Dotted => 'dotted',
      ::CSVPlusPlus::Modifier::BorderStyle::Double => 'Double',
      ::CSVPlusPlus::Modifier::BorderStyle::Solid => 'SOLId',
      ::CSVPlusPlus::Modifier::BorderStyle::SolidMedium => 'SOLID_MEDIUM',
      ::CSVPlusPlus::Modifier::BorderStyle::SolidThick => 'solid_thick'
    }.each do |expected_value, value|
      context value do
        before { modifier_validator.borderstyle = value }

        it { is_expected.to(eq(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.borderstyle = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#color=' do
    subject { modifier.color }

    context '#FF00AA' do
      before { modifier_validator.color = '#FF00AA' }

      it { is_expected.to(be_a(::CSVPlusPlus::Color)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.color = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#expand=' do
    subject { modifier.expand }

    context '5' do
      before { modifier_validator.expand = '5' }

      it { is_expected.to(be_a(::CSVPlusPlus::Modifier::Expand)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.expand = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#fontcolor=' do
    subject { modifier.fontcolor }

    context '#F0A' do
      before { modifier_validator.fontcolor = '#F0A' }

      it { is_expected.to(be_a(::CSVPlusPlus::Color)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.fontcolor = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#fontfamily=' do
    subject { modifier.fontfamily }

    context 'Helvetica Sans' do
      before { modifier_validator.fontfamily = 'Helvetica Sans' }

      it { is_expected.to(eq('Helvetica Sans')) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.fontfamily = '>>> Invalid$$|,,' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end

    context 'when quoted' do
      before { modifier_validator.fontfamily = "'Helvetica Sans'" }

      it { is_expected.to(eq('Helvetica Sans')) }
    end
  end

  describe '#fontsize=' do
    subject { modifier.fontsize }

    context '22' do
      before { modifier_validator.fontsize = '22' }

      it { is_expected.to(eq(22)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.fontsize = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#format=' do
    subject { modifier.formats }

    {
      ::CSVPlusPlus::Modifier::TextFormat::Bold => 'bold',
      ::CSVPlusPlus::Modifier::TextFormat::Italic => 'Italic',
      ::CSVPlusPlus::Modifier::TextFormat::Strikethrough => 'STRIKETHROUGH',
      ::CSVPlusPlus::Modifier::TextFormat::Underline => 'Underline'
    }.each do |expected_value, value|
      context value do
        before { modifier_validator.format = value }

        it { is_expected.to(include(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.format = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#freeze!' do
    subject { modifier }

    before { modifier_validator.freeze! }

    it { is_expected.to(be_frozen) }
  end

  describe '#halign=' do
    subject { modifier.halign }

    {
      ::CSVPlusPlus::Modifier::HorizontalAlign::Left => 'left',
      ::CSVPlusPlus::Modifier::HorizontalAlign::Center => 'CENTER',
      ::CSVPlusPlus::Modifier::HorizontalAlign::Right => 'Right'
    }.each do |expected_value, value|
      context value do
        before { modifier_validator.halign = value }

        it { is_expected.to(eq(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.halign = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#note=' do
    subject { modifier.note }

    context "'this is a note'" do
      before { modifier_validator.note = 'this is a note' }

      it { is_expected.to(eq('this is a note')) }
    end
  end

  describe '#numberformat=' do
    subject { modifier.numberformat }

    {
      ::CSVPlusPlus::Modifier::NumberFormat::Currency => 'currency',
      ::CSVPlusPlus::Modifier::NumberFormat::Date => 'Date',
      ::CSVPlusPlus::Modifier::NumberFormat::DateTime => 'DATE_TIME',
      ::CSVPlusPlus::Modifier::NumberFormat::Number => 'Number',
      ::CSVPlusPlus::Modifier::NumberFormat::Percent => 'PERCENT',
      ::CSVPlusPlus::Modifier::NumberFormat::Text => 'text',
      ::CSVPlusPlus::Modifier::NumberFormat::Time => 'time',
      ::CSVPlusPlus::Modifier::NumberFormat::Scientific => 'scientific'
    }.each do |expected_value, value|
      context value do
        before { modifier_validator.numberformat = value }

        it { is_expected.to(eq(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.numberformat = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#valign=' do
    subject { modifier.valign }

    {
      ::CSVPlusPlus::Modifier::VerticalAlign::Top => 'top',
      ::CSVPlusPlus::Modifier::VerticalAlign::Center => 'Center',
      ::CSVPlusPlus::Modifier::VerticalAlign::Bottom => 'BOTTOM'
    }.each do |expected_value, value|
      context value do
        before { modifier_validator.valign = value }

        it { is_expected.to(eq(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.valign = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#validate=' do
    subject { modifier.validate }

    context 'validation that takes no args' do
      before { modifier_validator.validate = 'blank' }

      it { is_expected.to(be_a(::CSVPlusPlus::Modifier::DataValidation)) }
    end

    context 'validation that takes two args' do
      before { modifier_validator.validate = 'number_eq: 42' }

      it { is_expected.to(be_a(::CSVPlusPlus::Modifier::DataValidation)) }
    end

    context 'validation that takes any number of args' do
      before { modifier_validator.validate = 'one_of_list: 1 2 3' }

      it { is_expected.to(be_a(::CSVPlusPlus::Modifier::DataValidation)) }
    end

    context 'an invalid validation' do
      it 'raises an error' do
        expect { modifier_validator.validate = 'foo: bar' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#var=' do
    subject { modifier.var }

    context 'variable_name' do
      before { modifier_validator.var = 'variable_name' }

      it { is_expected.to(eq(:variable_name)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier_validator.var = '@)V@)Xjk ask' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end
end
